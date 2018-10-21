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
    router.get("dice_run", dice_run, "GET");
    
    Iron::new(router).http("localhost:3000").unwrap();
}

fn dice_run(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();

    let mut dice: i32 = 0;
    let mut number: i32 = 0;
    if let Some(&Value::String(ref d)) = map.find(&["dice"]) {
        dice = d.parse().unwrap();
    }
    if let Some(&Value::String(ref num)) = map.find(&["number"]) {
        number = num.parse().unwrap();
    }
    println!("dice {}, number{}", dice, number);

    let result = Result {
        dice: dice,
        number: number,
        result: roll(dice, number),
    };
    let payload = json::encode(&result).unwrap();
    Ok(Response::with((ContentType::json().0, status::Ok, payload)))

}

fn roll(dice: i32, number: i32) -> i32 {
    let mut result: i32 = 0;

    for _ in 0..number {
        result += rand::thread_rng().gen_range(1, dice);
    }

    result
}
