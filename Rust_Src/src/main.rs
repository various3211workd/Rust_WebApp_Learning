extern crate iron;
extern crate router;

use std::io::Read;
 
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use router::Router;

fn main() {
    let mut router = Router::new();
    //router.get("/", return_to_GET, "GET");
    router.get("json", json, "GET");
    router.post("/", return_to_POST, "POST");

    Iron::new(router).http("localhost:3000").unwrap();
}

fn json(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((ContentType::json().0, status::Ok, "{}")))
}

fn return_to_GET(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((ContentType::json().0, status::Ok, "Hello World!")))
}

fn return_to_POST(req: &mut Request) -> IronResult<Response>{
    let mut body = String::new();
    req.body.read_to_string(&mut body).expect("error");

    let res = "return to POST <<".to_string() + &body + &">>".to_string();
    
    Ok(Response::with((status::Ok, res)))
}
