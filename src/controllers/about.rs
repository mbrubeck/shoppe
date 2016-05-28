
use iron::prelude::*;
use iron::status;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "This is a test")))
}


