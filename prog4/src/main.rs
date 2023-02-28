#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "ANDREW CHEATED ON 342 MIDTERM!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
