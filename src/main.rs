extern crate iron;
#[macro_use] extern crate router;
extern crate handlebars_iron as hbs;

use iron::prelude::*;
use iron::status;
use hbs::{Template, HandlebarsEngine, DirectorySource};

/// Entry point
fn main() {
    //
    // Constants
    //
    let addr = "localhost:3000";
    let templates = "./templates";

    // Routes
    let mut chain = Chain::new(router! {
        get "/" => index,
    });

    // Compile templates
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new(templates, ".hbs")));
    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        use std::error::Error;
        panic!("{}", r.description());
    }
    // Register handlebar middleware
    chain.link_after(hbse);

    // Fire & Forget
    println!("\nServer running at \x1b[33mhttp://{}\x1b[0m\n", addr);
    Iron::new(chain).http(addr).unwrap();
}

/// `/` Handler
fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", ())).set_mut(status::Ok);
    Ok(resp)
}
