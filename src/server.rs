use rocket_contrib::templates::Template;

use crate::routes::get_routes;

pub fn start_server() -> rocket::Rocket {
    let (static_files, routes, catchers) = get_routes();
    rocket::ignite()
        .mount("/", routes)
        .mount("/static", static_files)
        .register(catchers)
        .attach(Template::fairing())
}
