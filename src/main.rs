#![feature(concat_idents)]


extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

#[macro_use]
mod macros;
mod controllers;
mod error;

fn main() {
    use controllers::user;
    let mut router = Router::new();
    router.get("/", controllers::root::handler);
    router.get("/about", controllers::about::handler);

    resource!(router, "user", user);

    Iron::new(router).http("0.0.0.0:3000").unwrap();
}

