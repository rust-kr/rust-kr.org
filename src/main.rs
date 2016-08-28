#[macro_use] extern crate iron;
#[macro_use] extern crate router;
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

/// Entry point
fn main() {
    //
    // Configs
    //
    let addr = "localhost:8000";

    // Routes & Mounts
    let mut chain = Chain::new(router! {
        get "/" => index,
        get "/pages/:name" => page,
    });

    //
    // 이 뒤는 더럽습니다
    // Reference: https://namu.wiki/w/%EC%96%B4%EC%9D%B4%20%EA%B7%B8%20%EC%95%9E%EC%9D%80%20%EC%A7%80%EC%98%A5%EC%9D%B4%EB%8B%A4
    //

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
    println!("\nServer running at \x1b[33mhttp://{}\x1b[0m\n", addr);
    Iron::new(mount).http(addr).unwrap();
}

/// Root page handler
fn index(_: &mut Request) -> IronResult<Response> {
    let mut data = BTreeMap::new();
    data.insert("content", "Hello, world!");

    Ok(Response::with((status::Ok, Template::new("default", data))))
}

/// Wiki page handler
fn page(req: &mut Request) -> IronResult<Response> {
    use std::fs::File;
    use std::io::Read;
    use router::Router;
    use cmark::Parser;
    use cmark::html::push_html;

    // Parse URL
    let name = req.extensions.get::<Router>().unwrap().find("name").unwrap_or("Home");

    // Read file
    let path = format!("docs/{}.md", name);
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
