extern crate iron;
extern crate router;

use std::io::Read;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main(){
    let mut router = Router::new();
    router.post("/", return_to_POST, "POST");

    Iron::new(router).http("localhost:80").unwrap();

    fn return_to_POST(req: &mut Request) -> IronResult<Response>{
        let mut body = String::new();
        req.body.read_to_string(&mut body).expect("error");
        
        let res = "return to POST <<".to_string() + &body + &">>".to_string();
        
        Ok(Response::with((status::Ok, res)))
    }
}