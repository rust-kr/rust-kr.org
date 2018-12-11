//! [rust-kr.org] [![Docker Badge]][Docker Hub]
//! ========
//! 한국 러스트 사용자 그룹 홈페이지의 소스코드 입니다.
//!
//! ```bash
//! docker run --detach \
//!   --name rust-kr.org \
//!   --restart always \
//!   --publish 127.0.0.1:8000:8000 \
//!   simnalamburt/rust-kr.org
//!   snu.jokbo.me
//! ```
//!
//! [rust-kr.org]: https://rust-kr.org
//! [Docker Badge]: https://badgen.net/docker/pulls/simnalamburt/rust-kr.org?icon=docker&label=pulls
//! [Docker Hub]: https://hub.docker.com/r/simnalamburt/rust-kr.org/

// rust-kr.org
// Copyright (C) 2016-2018  Hyeon Kim
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::BTreeMap;
use iron::{prelude::*, status, itry};
use iron::middleware::{Handler, AfterMiddleware};
use staticfile::Static;
use handlebars_iron::{Template, HandlebarsEngine, DirectorySource};
use router::router;

//
// Configs
//
/// 서버가 소켓을 열 주소
const ADDR: &'static str = "[::]:8000";
/// 마크다운 문서가 담긴 주소
const DOCS_DIR: &'static str = "docs";

/// Entry point
fn main() {
    //
    //      User sends request
    //               ⇩
    //                                '/public' 디렉토리에서 해당 파일이 있는지 먼저 찾아본다.
    //    try static file serving     있을경우 이를 전송하고, 없을경우 아래의 404 page handler로
    //                                넘어간다.
    //               ⇩
    //        "main handlers"         index, page, all_docs 등의 함수로 요청을 처리한다
    //               ⇩
    //       404 page handler         위의 두개가 모두 실패하였을경우 404페이지를 렌더링한다
    //               ⇩
    //     handlebar templating
    //

    let mut c = Chain::new(router! {
        index: get "/" => index,
        page: get "/pages/:name" => page,
        all_docs: get "/pages/_pages" => all_docs,
    });

    // Static file serving
    c.link_around(move |main: Box<dyn Handler>| -> Box<dyn Handler> {
        Box::new(move |req: &mut Request| -> IronResult<Response> {
            Static::new("public/").handle(req)
                .or_else(|_| main.handle(req))
        })
    });
    // 404 page handler
    c.link_after(catch(|mut err: IronError| {
        if err.response.body.is_some() { return Err(err); }
        if err.response.status != Some(status::NotFound) { return Err(err); }

        // TODO: 좀더 내용 채우기
        let mut data = BTreeMap::new();
        data.insert("content", r#"
            <h1>404</h1>
            <p>This is not the web page you are looking for.</p>
        "#);
        err.response.set_mut(Template::new("default", data));
        Err(err)
    }));
    // Handlebar templating
    c.link_after({
        let mut hbr = HandlebarsEngine::new();
        hbr.add(Box::new(DirectorySource::new("templates", ".hbs")));
        if let Err(r) = hbr.reload() {
            use std::error::Error;

            panic!("{}", r.description());
        }
        hbr
    });

    println!("\nServer running at \x1b[32mhttp://{}\x1b[0m\n", ADDR);
    Iron::new(c).http(ADDR).unwrap();
}

/// Root page handler
fn index(_: &mut Request) -> IronResult<Response> {
    read_docs("README")
}

/// Wiki page handler
fn page(req: &mut Request) -> IronResult<Response> {
    use router::Router;

    // Parse URL
    let name = req.extensions.get::<Router>().unwrap().find("name").unwrap_or("README");
    read_docs(name)
}

/// "All documents" page handler
fn all_docs(_: &mut Request) -> IronResult<Response> {
    use std::fs::read_dir;

    let mut paths: Vec<_> = itry!(read_dir(DOCS_DIR), status::NotFound)
        .filter_map(|f| f.ok())
        .map(|f| f.path())
        .filter(|p| p.is_file())
        .filter(|p| p.as_os_str().to_str()
            .map(|s| s.ends_with(".md"))
            .unwrap_or(false))
        .collect();
    paths.sort();

    let pages = paths.iter()
        .filter_map(|p| p.file_stem())
        .filter_map(|stem| stem.to_str());

    // TODO: 러스트로 이짓하지 말고 Handlebar로 대체
    let mut html = "<ul>".to_string();
    for page in pages {
        use std::fmt::Write;

        write!(&mut html, r#"<li><a href="/pages/{0}">{0}</a></li>"#, page).unwrap();
    }
    html.push_str("</ul>");

    // Fill in to the template
    let mut data = BTreeMap::new();
    data.insert("content", html);
    Ok(Response::with((status::Ok, Template::new("default", data))))
}

/// `name`을 인자로 받아, `docs/<name>.md` 파일을 렌더링하여 `IronResult<Response>`로 반환합니다.
fn read_docs(name: &str) -> IronResult<Response> {
    use std::{fs::File, io::Read};
    use pulldown_cmark::{Parser, html::push_html};

    // Read file
    let path = format!("{}/{}.md", DOCS_DIR, name);
    let mut file = itry!(File::open(&path), status::NotFound);
    let mut md = String::new();
    itry!(file.read_to_string(&mut md), status::ServiceUnavailable);

    // Parse markdown
    let parser = Parser::new(&md);
    let mut html = String::new();
    push_html(&mut html, parser);

    // Fill in to the template
    let mut data = BTreeMap::new();
    data.insert("content", html);
    Ok(Response::with((status::Ok, Template::new("default", data))))
}

/// Helper struct for handle 404 page of rust-kr
struct RustkrHandler<F> { handler: F }
/// Helper function for handle 404 page of rust-kr
fn catch<F>(handler: F) -> RustkrHandler<F> { RustkrHandler { handler: handler } }
impl<F> AfterMiddleware for RustkrHandler<F>
    where F: Send + Sync + 'static + Fn(IronError) -> IronResult<Response> {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        (self.handler)(err)
    }
}
