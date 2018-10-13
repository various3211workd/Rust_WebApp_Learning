extern crate iron;
extern crate router;
extern crate params;
extern crate rand;
extern crate rustc_serialize;
 
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use router::Router;
use params::{Params, Value};
use rand::Rng;

use rustc_serialize::json;

#[derive(RustcEncodable)]
pub struct Result {
    dice: i32,
    number: i32,
    result: i32,
}

fn main() {
    let mut router = Router::new();
    router.get("dice?run", dice_run, "GET");
    
    Iron::new(router).http("localhost:3000").unwrap();
}

fn dice_run(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();

    match map.find(&["dice"]) {
        Some(&Value::String(ref dice)) => {
            let dice: i32 = dice.parse().unwrap();
            //let number: i32 = number.parse().unwrap();
            let number: i32 = 1;
            let result = Result {
                dice: dice,
                number: number,
                result: roll(dice, number),
                //text: "ather text".to_string(),
            };
            let payload = json::encode(&result).unwrap();
            Ok(Response::with((ContentType::json().0, status::Ok, payload)))
        },
        _ => Ok(Response::with((status::Ok, "Hello world")))
    }
}

fn roll(dice: i32, number: i32) -> i32 {
    let mut result: i32 = 0;

    for _ in 0..number {
        result += rand::thread_rng().gen_range(1, dice);
    }

    result
}
