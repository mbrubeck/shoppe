use iron::prelude::*;

use std::error::Error;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct NotImplemented {
    route: String,
}

impl NotImplemented {
    pub fn new(req: &Request) -> NotImplemented {
        NotImplemented {
            route: req.url.clone().into_generic_url().serialize()
        }
    }
}

impl fmt::Display for NotImplemented {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for NotImplemented {
    fn description(&self) -> &str { &self.route }
}

