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
    let context = get_base_context("/");
    dbg!(&context);
    Template::render("index", context)
}

#[get("/resume_pdf")]
fn resume_pdf() -> std::io::Result<NamedFile> {
    NamedFile::open(get_template("/resume_pdf"))
}

#[get("/resume")]
fn resume() -> Template {
    let context = get_base_context("/resume");
    Template::render(get_template("/resume"), context)
}

#[get("/500")]
fn crash() -> Result<String, Status> {
    Err(Status::InternalServerError)
}

#[get("/blog")]
fn blog_index() -> Template {
    let context = get_base_context("/blog");
    Template::render(get_template("/blog"), context)
}

#[get("/blog/<slug>")]
fn blog_article(slug: String) -> Option<Template> {
    let mut context = get_base_context("/blog");
    match context.blog.html.get(&slug) {
        Some(curr_blog) => {
            context.curr_blog = Some(curr_blog);
            context.kv.insert("curr_slug".to_owned(), slug);
            Some(Template::render("blog/blog_article", context))
        }
        None => None,
    }
}

#[get("/linkedin")]
fn linkedin() -> Template {
    let context = get_base_context("/linkedin");
    Template::render(get_template("/linkedin"), context)
}

#[get("/github")]
fn github() -> Template {
    let context = get_base_context("/github");
    Template::render(get_template("/github"), context)
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut context = get_base_context("/");
    context.kv.insert("uri".to_owned(), req.uri().to_string());
    Template::render(get_template("404"), context)
}

#[catch(500)]
fn server_err(req: &Request) -> Template {
    let mut context = get_base_context("/");
    context.kv.insert("uri".to_owned(), req.uri().to_string());
    Template::render(get_template("500"), context)
}

pub fn get_routes() -> (StaticFiles, Vec<Route>, Vec<Catcher>) {
    (
        StaticFiles::from("static"),
        routes![
            index,
            crash,
            resume,
            blog_index,
            linkedin,
            github,
            resume_pdf,
            blog_article
        ],
        catchers![server_err, not_found],
    )
}
