use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::Request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use crate::context::{get_base_context, get_template};
use rocket::Catcher;
use rocket::Route;

// And just like that, months and months later,
// it just works.
macro_rules! simple_route {
    ($name:ident, $route:literal, $title:literal) => {
        #[get($route)]
        fn $name() -> Template {
            let mut context = get_base_context($route);
            context.kv.insert("title".to_owned(), $title.to_owned());
            Template::render(get_template($route), context)
        }
    };
}

simple_route! {index, "/", "home"}
simple_route! {resume, "/resume", "resume"}
simple_route! {blog_index, "/blog", "blog"}
simple_route! {linkedin, "/linkedin", "linkedin"}
simple_route! {github, "/github", "github"}

#[get("/robots.txt")]
fn robots_txt() -> std::io::Result<&'static str> {
    // NamedFile::open(get_template("/resume_pdf"))
    let robots_txt = r#"
# robots.txt
User-agent: *
Disallow:
"#;
    Ok(robots_txt)
}

#[get("/resume_pdf")]
fn resume_pdf() -> std::io::Result<NamedFile> {
    NamedFile::open(get_template("/resume_pdf"))
}

#[get("/500")]
fn crash() -> Result<String, Status> {
    Err(Status::InternalServerError)
}

#[get("/blog/<slug>")]
fn blog_article(slug: String) -> Option<Template> {
    let mut context = get_base_context("/blog");
    context.kv.insert("title".to_owned(), "blog".to_owned());
    context.blog.html.get(&slug).map(|curr_blog| {
        context.curr_blog = Some(curr_blog);
        context.kv.insert("curr_slug".to_owned(), slug);
        Template::render("blog/blog_article", context)
    })
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut context = get_base_context("/");
    context.kv.insert("uri".to_owned(), req.uri().to_string());
    context.kv.insert("title".to_owned(), "404".to_owned());
    Template::render(get_template("404"), context)
}

#[catch(500)]
fn server_err(req: &Request<'_>) -> Template {
    let mut context = get_base_context("/");
    context.kv.insert("uri".to_owned(), req.uri().to_string());
    context.kv.insert("title".to_owned(), "500".to_owned());
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
            robots_txt,
            blog_article
        ],
        catchers![server_err, not_found],
    )
}
