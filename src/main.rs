
#[macro_use] extern crate rocket;
extern crate tfc;

use rocket::{response::{status, content}, http::Status};
use std::{thread, time::Duration};
use tfc::{Context, traits::*};

#[post("/", data = "<d>")]
async fn receive(d: String) -> Result<status::Accepted<&'static str>, status::Custom<content::RawText<&'static str>>>  {
    println!("I got {}", d);

    if d.len() == 0 { return Err(status::Custom(Status::BadRequest, content::RawText("No data"))) }

    let mut ctx = match Context::new() {
        Ok(ctx) => ctx,
        Err(err) => panic!("err {:#?}", err),
    };

    // For OS-specific reasons, it's necessary to wait a moment after
    // creating the context before generating events.
    thread::sleep(Duration::from_millis(350));
    match ctx.ascii_string(d.as_bytes()) {
        Ok(_) => Ok(status::Accepted(Some(""))),
        Err(_) => Err(status::Custom(Status::BadRequest, content::RawText("Bad data")))
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