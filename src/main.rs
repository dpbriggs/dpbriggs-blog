#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#[macro_use]
extern crate rocket;
extern crate lazy_static;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod blog;
mod context;
mod routes;
mod server;

use blog::get_org_mode_files;
use server::start_server;

fn main() {
    get_org_mode_files();
    start_server().launch();
}
