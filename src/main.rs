extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(move |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("localhost:3000").unwrap();
}
