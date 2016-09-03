#[macro_use(itry)] extern crate iron;
#[macro_use(router)] extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;
extern crate pulldown_cmark as cmark;

use std::collections::BTreeMap;
use iron::prelude::*;
use iron::status;
use mount::Mount;
use staticfile::Static;
use hbs::{Template, HandlebarsEngine, DirectorySource};

//
// Configs
//
/// 서버가 소켓을 열 주소
const ADDR: &'static str = "localhost:8000";
/// 마크다운 문서가 담긴 주소
const DOCS_DIR: &'static str = "docs";

/// Entry point
fn main() {
    let mut chain = Chain::new(router! {
        get "/" => index,
        get "/pages/:name" => page,
        get "/pages/_pages" => all_docs,
    });

    // Compile templates
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("templates", ".hbs")));
    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        use std::error::Error;
        panic!("{}", r.description());
    }
    // Register handlebar middleware
    chain.link_after(hbse);

    // Mount '/static/' path to the filesystem
    let mut mount = Mount::new();
    mount
        .mount("/", chain)
        .mount("/static/", Static::new("static/"));

    // Fire & Forget
    println!("\nServer running at \x1b[33mhttp://{}\x1b[0m\n", ADDR);
    Iron::new(mount).http(ADDR).unwrap();
}

/// Root page handler
fn index(_: &mut Request) -> IronResult<Response> {
    read_docs("Home")
}

/// Wiki page handler
fn page(req: &mut Request) -> IronResult<Response> {
    use router::Router;

    // Parse URL
    let name = req.extensions.get::<Router>().unwrap().find("name").unwrap_or("Home");
    read_docs(name)
}

/// "All documents" page handler
fn all_docs(_: &mut Request) -> IronResult<Response> {
    use std::fs::read_dir;

    let mut paths: Vec<_> = itry!(read_dir(DOCS_DIR))
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
    use std::fs::File;
    use std::io::Read;
    use cmark::Parser;
    use cmark::html::push_html;

    // Read file
    let path = format!("{}/{}.md", DOCS_DIR, name);
    let mut file = itry!(File::open(&path));
    let mut md = String::new();
    itry!(file.read_to_string(&mut md));

    // Parse markdown
    let parser = Parser::new(&md);
    let mut html = String::new();
    push_html(&mut html, parser);

    // Fill in to the template
    let mut data = BTreeMap::new();
    data.insert("content", html);
    Ok(Response::with((status::Ok, Template::new("default", data))))
}
