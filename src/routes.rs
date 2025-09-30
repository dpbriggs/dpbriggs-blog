use crate::blog::OrgBlog;
use crate::context::get_base_context;
use crate::error::SiteError;
use miette::Result;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};

pub fn generate_site(tera: &Tera, output_dir: &str, blog: &OrgBlog) -> Result<()> {
    // Helper function to render and write a file
    let render_and_write =
        |template_name: &str, context: &Context, output_path: &str| -> Result<()> {
            println!("Rendering {} to {}", template_name, output_path);
            let content = tera
                .render(template_name, context)
                .map_err(SiteError::from)?;
            let path = Path::new(output_dir).join(output_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(SiteError::from)?;
            }
            let mut file = File::create(path).map_err(SiteError::from)?;
            file.write_all(content.as_bytes())
                .map_err(SiteError::from)?;
            Ok(())
        };

    // Generate index page
    let mut context = get_base_context("/", blog);
    context.kv.insert("title".to_owned(), "home".into());
    render_and_write("index.html.tera", &(&context).into(), "index.html")?;

    // Generate resume page
    let mut context = get_base_context("/resume", blog);
    context.kv.insert("title".to_owned(), "resume".into());
    render_and_write("resume.html.tera", &(&context).into(), "resume/index.html")?;

    // Generate blog index page
    let mut context = get_base_context("/blog", blog);
    context.kv.insert("title".to_owned(), "blog".into());
    render_and_write(
        "blog/blog_root.html.tera",
        &(&context).into(),
        "blog/index.html",
    )?;

    // Generate linkedin page
    let mut context = get_base_context("/linkedin", blog);
    context.kv.insert("title".to_owned(), "linkedin".into());
    render_and_write(
        "linkedin.html.tera",
        &(&context).into(),
        "linkedin/index.html",
    )?;

    // Generate github page
    let mut context = get_base_context("/github", blog);
    context.kv.insert("title".to_owned(), "github".into());
    render_and_write("github.html.tera", &(&context).into(), "github/index.html")?;

    // Generate robots.txt
    println!("Generating robots.txt");
    let mut file =
        File::create(Path::new(output_dir).join("robots.txt")).map_err(SiteError::from)?;
    file.write_all(b"User-agent: *\nDisallow:")
        .map_err(SiteError::from)?;

    // Generate RSS feed
    let rss_context = get_base_context("/blog", blog);
    render_and_write(
        "blog-rss.xml.tera",
        &(&rss_context).into(),
        "feed/index.xml",
    )?;

    // Generate 404 page
    let mut context = get_base_context("/", blog);
    context.kv.insert("title".to_owned(), "404".into());
    context.kv.insert("blog_uri".to_owned(), "".into());
    render_and_write("404.html.tera", &(&context).into(), "404.html")?;

    // Generate 500 page
    let mut context = get_base_context("/", blog);
    context.kv.insert("title".to_owned(), "500".into());
    context.kv.insert("uri".to_owned(), "/".into());
    render_and_write("500.html.tera", &(&context).into(), "500.html")?;

    // Generate blog articles
    for (slug, blog_post) in &blog.html {
        let mut context = get_base_context("/blog", blog);
        context.kv.insert("title".to_owned(), "blog".to_owned());
        context.curr_blog = Some(blog_post);
        context.kv.insert("curr_slug".to_owned(), slug.clone());
        let output_path = format!("blog/{}/index.html", slug);
        render_and_write(
            "blog/blog_article.html.tera",
            &(&context).into(),
            &output_path,
        )?;
    }

    Ok(())
}
