#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;
use rand::Rng;

static mut expected: u64 = 10;


#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
        format!("Hello, {} year old named {}!", age, name)
}

#[get("/guess/<num>")]
fn guess(num: u64) -> String {
    unsafe {
        if num > expected {
            return format!("bigger");
        } else if num < expected {
            return format!("smaller");
        } else {
            return format!("correct");
        }
    }
}

fn main() {
        unsafe {
            expected = rand::thread_rng().gen_range(1, 101);
        }
        rocket::ignite().mount("/", routes![hello, guess]).launch();
}

