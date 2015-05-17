use iron::prelude::*;
use iron::status;

/// Test hello world server
pub fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
}
