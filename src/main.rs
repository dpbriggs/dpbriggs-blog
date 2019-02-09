#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#[macro_use]
extern crate rocket;
extern crate lazy_static;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

mod blog;
mod context;
mod routes;
mod server;

use server::start_server;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    start_server().launch();
}
