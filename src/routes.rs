use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::Request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use crate::context::{get_base_context, get_template};
use rocket::Catcher;
use rocket::Route;

#[get("/")]
fn index() -> Template {
    let context = get_base_context();
    Template::render("index", context)
}

#[get("/resume")]
fn resume() -> std::io::Result<NamedFile> {
    NamedFile::open(get_template("/resume"))
}

#[get("/500")]
fn crash() -> Result<String, Status> {
    Err(Status::InternalServerError)
}

#[get("/blog")]
fn blog_index() -> Template {
    let mut context = get_base_context();
    context.insert("nav_site_href".to_owned(), "/blog".to_string());
    Template::render(get_template("/blog"), context)
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut context = get_base_context();
    context.insert("uri".to_owned(), req.uri().to_string());
    Template::render(get_template("404"), context)
}

#[catch(500)]
fn server_err(req: &Request) -> Template {
    let mut context = get_base_context();
    context.insert("uri".to_owned(), req.uri().to_string());
    Template::render(get_template("500"), context)
}

pub fn get_routes() -> (StaticFiles, Vec<Route>, Vec<Catcher>) {
    (
        StaticFiles::from("static"),
        routes![index, crash, resume, blog_index],
        catchers![server_err, not_found],
    )
}