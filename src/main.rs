#[macro_use] extern crate rocket;
extern crate tfc;

use std::{thread, time::Duration};
use tfc::{Context, traits::*};

#[post("/", data = "<d>")]
async fn receive(d: String) -> &'static str {
    println!("I got {}", d);

    if (d.len() == 0) { return "Man that's null" }

    let mut ctx = match Context::new() {
        Ok(ctx) => ctx,
        Err(err) => panic!("err {:#?}", err),
    };

    // For OS-specific reasons, it's necessary to wait a moment after
    // creating the context before generating events.
    thread::sleep(Duration::from_millis(950));
    match ctx.ascii_string(d.as_bytes()) {
        Ok(_) => "Thanks man",
        Err(_) => "Couldn't type"
    }

}

#[get("/")]
fn index() -> &'static str {
    "QRCord, you shouldn't be here"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, receive])
}