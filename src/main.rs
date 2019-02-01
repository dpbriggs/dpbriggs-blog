#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

type Foo = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct FooBar {
    foo: u64,
    bar: String,
}

#[get("/foobar/<age>/<name>")]
fn foobar(age: u64, name: String) -> String {
    let f = FooBar {
        foo: age,
        bar: name,
    };
    serde_json::to_string_pretty(&f).unwrap()
}

fn get_base_context() -> Foo {
    let mut context = Foo::new();
    context.insert("site_name".to_owned(), "dpbriggs".to_owned());
    context
}

#[get("/")]
fn index() -> Template {
    let context = get_base_context();
    Template::render("index", context)
}

fn main() {
    let static_files = StaticFiles::from("static");
    let routes = routes![foobar, index];
    rocket::ignite()
        .mount("/", routes)
        .mount("/static", static_files)
        .attach(Template::fairing())
        .launch();
}
