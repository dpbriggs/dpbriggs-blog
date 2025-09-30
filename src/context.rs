use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::HashMap;

use crate::blog::{OrgBlog, OrgModeHtml};

/// BLOG_ROOT is the relative path to blog
pub static BLOG_ROOT: &str = "blog/";

/// SiteContextKv represents all key-value variables used in
/// this project.
///
/// # Example
///
/// let mut foo = SiteContextKv::new()
/// foo.insert("key".to_owned(), "value".to_owned())
type SiteContextKv = HashMap<String, String>;

/// SiteContext represents the entire context required to render
/// this website. See [get_base_context](crate::context::get_base_context)
#[derive(Serialize, Debug)]
pub struct SiteContext<'a> {
    /// base is the static key-value context of the website.
    /// All of the information in base comes from
    /// [STATIC_SITE_CONTEXT_KV](crate::context::STATIC_SITE_CONTEXT_KV)
    pub base: &'static SiteContextKv,
    /// kv is the dynamic key-value context of the website.
    pub kv: SiteContextKv,
    /// blog is all blog related items, see [OrgBlog](crate::context::OrgBlog)
    pub blog: &'a OrgBlog,
    /// curr_blog is the current blog article, if applicable.
    pub curr_blog: Option<&'a OrgModeHtml>,
}

macro_rules! site_context(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = SiteContextKv::new();
            $(
                m.insert($key.to_owned(), $value.to_owned());
            )+
                m
        }
    };
);

lazy_static! {
    static ref STATIC_SITE_CONTEXT_KV: SiteContextKv = {
        site_context! {
            "domain_name" =>  "dpbriggs.ca",
            "nav_site_href" =>  "/",
            "root_uri" =>  "/",
            "blog_uri" =>  "/blog",
            "resume_uri" =>  "/resume",
            "linkedin_uri" =>  "/linkedin",
            "github_uri" =>  "/github",
            "resume_uri" =>  "/resume",
            "resume_pdf_uri" =>  "/dpbriggs_resume.pdf",
            "rss_uri" =>  "/feed/index.xml",
            "crash_uri" =>  "/500",
            "web_sep" =>  "--",
            "admin_email" =>  "david@dpbriggs.ca",
            "full_name" =>  "David Briggs",
            "internet_handle" =>  "dpbriggs",
            "my_email" =>  "david@dpbriggs.ca",
            "github_url" => "https://github.com/dpbriggs",
            "github_repo_url" => "https://github.com/dpbriggs/dpbriggs-blog",
            "linkedin_url" => "https://www.linkedin.com/in/dpbriggs"
        }
    };
}

use tera::Context;

impl<'a> From<&SiteContext<'a>> for Context {
    fn from(site_context: &SiteContext<'a>) -> Self {
        let mut context = Context::new();
        context.insert("base", &site_context.base);
        context.insert("kv", &site_context.kv);
        context.insert("blog", &site_context.blog);
        context.insert("curr_blog", &site_context.curr_blog);
        context
    }
}

/// get_base_context
pub fn get_base_context<'a>(nav_href_uri: &str, blog: &'a OrgBlog) -> SiteContext<'a> {
    SiteContext {
        base: &STATIC_SITE_CONTEXT_KV,
        // TODO: Not waste memory like this.
        kv: {
            let mut tmp = SiteContextKv::new();
            tmp.insert("nav_site_href".to_owned(), nav_href_uri.to_owned());
            tmp
        },
        blog,
        curr_blog: None,
    }
}
