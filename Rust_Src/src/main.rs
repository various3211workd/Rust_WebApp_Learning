extern crate iron;
extern crate router;

use std::io::Read;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main(){
    let mut router = Router::new();
    router.get("/", hello, "hello");
    router.post("/", hello_prefix, "hello_prefix");

    Iron::new(router).http("localhost:3000").unwrap();

    fn hello(_: &mut Request) -> IronResult<Response>{
        Ok(Response::with((status::Ok, "Hello World")))
    }

    fn hello_prefix(req: &mut Request) -> IronResult<Response>{
        let mut body = String::new();
        req.body.read_to_string(&mut body)
            .expect("Failed to read line");
        
        let res = "Hello ".to_string() + &body + &"!".to_string();
        Ok(Response::with((status::Ok, res)))
    }
}