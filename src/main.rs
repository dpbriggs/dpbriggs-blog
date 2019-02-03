#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
use rocket::http::Status;
use rocket::Request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use std::collections::HashMap;

type SiteContext = HashMap<String, String>;

fn get_base_context() -> SiteContext {
    let mut context = SiteContext::new();
    context.insert("nav_site_name".to_owned(), "dpbriggs.ca".to_owned());
    context.insert("nav_site_href".to_owned(), "/".to_owned());
    context.insert("root_uri".to_owned(), "/".to_owned());
    context.insert("blog_uri".to_owned(), "/blog".to_owned());
    context.insert("resume_uri".to_owned(), "/resume".to_owned());
    context.insert("crash_uri".to_owned(), "/500".to_owned());
    context.insert("admin_email".to_owned(), "email@dpbriggs.ca".to_owned());
    context.insert("my_email".to_owned(), "email@dpbriggs.ca".to_owned());
    context.insert(
        "github_url".to_owned(),
        "https://github.com/dpbriggs".to_owned(),
    );
    context
}

#[get("/")]
fn index() -> Template {
    let context = get_base_context();
    Template::render("index", context)
}

#[get("/500")]
fn crash() -> Result<String, Status> {
    Err(Status::InternalServerError)
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut context = get_base_context();
    context.insert("uri".to_owned(), req.uri().to_string());
    Template::render("404", context)
}

#[catch(500)]
fn server_err(req: &Request) -> Template {
    let mut context = get_base_context();
    context.insert("uri".to_owned(), req.uri().to_string());
    Template::render("500", context)
}

fn main() {
    let static_files = StaticFiles::from("static");
    let routes = routes![index, crash];
    rocket::ignite()
        .mount("/", routes)
        .mount("/static", static_files)
        .register(catchers![not_found, server_err])
        .attach(Template::fairing())
        .launch();
}
