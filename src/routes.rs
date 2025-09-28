use crate::context::get_base_context;
use axum::{
    Router,
    extract::{Extension, OriginalUri, Path},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use axum_template::RenderHtml;
use axum_template::{TemplateEngine, engine::Engine};
use tera::Tera;
use tower_http::services::ServeFile;

async fn redirect_trailing_slash(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    let uri = request.uri();
    let path = uri.path();

    // Remove trailing slash (except for root "/")
    if path.len() > 1 && path.ends_with('/') {
        let new_path = path.trim_end_matches('/');
        let new_uri = if let Some(query) = uri.query() {
            format!("{}?{}", new_path, query)
        } else {
            new_path.to_string()
        };

        return Redirect::permanent(&new_uri).into_response();
    }

    next.run(request).await
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/resume", get(resume))
        .route("/blog", get(blog_index))
        .route("/linkedin", get(linkedin))
        .route("/github", get(github))
        .route("/robots.txt", get(robots_txt))
        .route("/feed", get(feed))
        .route("/rss", get(rss))
        .route_service("/resume_pdf", ServeFile::new("resume/dpbriggs_resume.pdf"))
        .route("/500", get(crash))
        .route("/blog/{slug}", get(blog_article))
        .fallback(not_found)
        .layer(axum::middleware::from_fn(redirect_trailing_slash))
}

async fn index(Extension(engine): Extension<Engine<Tera>>) -> impl IntoResponse {
    let mut context = get_base_context("/");
    context.kv.insert("title".to_owned(), "home".into());
    RenderHtml("index.html.tera", engine, context)
}

async fn resume(Extension(engine): Extension<Engine<Tera>>) -> impl IntoResponse {
    let mut context = get_base_context("/resume");
    context.kv.insert("title".to_owned(), "resume".into());
    RenderHtml("resume.html.tera", engine, context)
}

async fn blog_index(Extension(engine): Extension<Engine<Tera>>) -> impl IntoResponse {
    let mut context = get_base_context("/blog");
    context.kv.insert("title".to_owned(), "blog".into());
    RenderHtml("blog/blog_root.html.tera", engine, context)
}

async fn linkedin(Extension(engine): Extension<Engine<Tera>>) -> impl IntoResponse {
    let mut context = get_base_context("/linkedin");
    context.kv.insert("title".to_owned(), "linkedin".into());
    RenderHtml("linkedin.html.tera", engine, context)
}

async fn github(Extension(engine): Extension<Engine<Tera>>) -> impl IntoResponse {
    let mut context = get_base_context("/github");
    context.kv.insert("title".to_owned(), "github".into());
    RenderHtml("github.html.tera", engine, context)
}

async fn robots_txt() -> impl IntoResponse {
    "User-agent: *\nDisallow:"
}

async fn feed(Extension(engine): Extension<Engine<Tera>>) -> Response {
    rss(Extension(engine)).await
}

async fn rss(Extension(engine): Extension<Engine<Tera>>) -> Response {
    let context = get_base_context("/blog");
    let template = RenderHtml("blog-rss.xml.tera", engine, context);
    (
        StatusCode::OK,
        [("Content-Type", "application/rss+xml")],
        template.into_response(),
    )
        .into_response()
}

async fn crash() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

async fn blog_article(
    original_uri: OriginalUri,
    Path(slug): Path<String>,
    Extension(engine): Extension<Engine<Tera>>,
) -> Response {
    let mut context = get_base_context("/blog");
    context.kv.insert("title".to_owned(), "blog".to_owned());
    if let Some(curr_blog) = context.blog.html.get(&slug) {
        context.curr_blog = Some(curr_blog);
        context.kv.insert("curr_slug".to_owned(), slug);
        RenderHtml("blog/blog_article.html.tera", engine, context).into_response()
    } else {
        not_found(original_uri, Extension(engine))
            .await
            .into_response()
    }
}

async fn not_found(
    OriginalUri(original_uri): OriginalUri,
    Extension(engine): Extension<Engine<Tera>>,
) -> impl IntoResponse {
    let mut context = get_base_context("/");
    context.kv.insert("title".to_owned(), "404".to_owned());
    context
        .kv
        .insert("uri".to_owned(), original_uri.path().to_string());
    context.kv.insert("blog_uri".to_owned(), "".to_owned());
    use miette::IntoDiagnostic;
    if let Err(e) = engine.render("404.html.tera", &context).into_diagnostic() {
        println!("Error serializing context for 404 page: {:?}", e);
    }
    (
        StatusCode::NOT_FOUND,
        RenderHtml("404.html.tera", engine, context),
    )
}
