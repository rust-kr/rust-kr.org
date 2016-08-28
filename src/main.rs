extern crate iron;
#[macro_use] extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;

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
    });


    //
    // 이 뒤는 더럽습니다
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

/// `/` Handler
fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", ())).set_mut(status::Ok);
    Ok(resp)
}
