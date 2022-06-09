#[macro_use] extern crate rocket;
extern crate tfc;

use std::{thread, time::Duration};
use tfc::{Context, traits::*};

#[post("/", data = "<paste>")]
async fn receive(paste: String) -> &'static str {
    println!("I got {}", paste);

    let mut ctx = match Context::new() {
        Ok(ctx) => ctx,
        Err(err) => panic!("err {:#?}", err),
    };

    // For OS-specific reasons, it's necessary to wait a moment after
    // creating the context before generating events.
    thread::sleep(Duration::from_millis(15));
    ctx.unicode_string("321");

    "Thanks man"
}

#[get("/")]
fn index() -> &'static str {
    "QRCord, you shouldn't be here"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, receive])
}