
use iron::prelude::*;
use iron::status;
use router::Router;

use error::NotImplemented;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");
    Ok(Response::with((status::Ok, *id)))
}


pub fn index(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}


