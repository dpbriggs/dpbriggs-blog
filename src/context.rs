use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::blog::{get_org_blog, OrgBlog, OrgModeHtml};

pub type SiteContextKv = HashMap<String, String>;
pub type SiteContextBlog = OrgBlog;
type TemplateMap = HashMap<&'static str, &'static str>;

#[derive(Serialize, Debug)]
pub struct SiteContext<'a> {
    pub base: &'static SiteContextKv,
    pub kv: SiteContextKv,
    pub blog: &'static SiteContextBlog,
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
    static ref STATIC_BLOG_ENTRIES: SiteContextBlog = get_org_blog();
}

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
            "resume_pdf_uri" =>  "/resume_pdf",
            "crash_uri" =>  "/500",
            "web_sep" =>  "--",
            "admin_email" =>  "email@dpbriggs.ca",
            "full_name" =>  "David Briggs",
            "internet_handle" =>  "dpbriggs",
            "my_email" =>  "email@dpbriggs.ca",
            "github_url" => "https://github.com/dpbriggs",
            "github_repo_url" => "https://github.com/dpbriggs/dpbriggs-blog",
            "linkedin_url" => "https://www.linkedin.com/in/dpbriggs"
        }
    };
}

pub fn get_base_context(nav_href_uri: &str) -> SiteContext {
    SiteContext {
        base: &STATIC_SITE_CONTEXT_KV,
        kv: {
            let mut tmp = SiteContextKv::new();
            tmp.insert("nav_site_href".to_owned(), nav_href_uri.to_owned());
            tmp
        },
        blog: &STATIC_BLOG_ENTRIES,
        curr_blog: None,
    }
}

macro_rules! template_map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = TemplateMap::new();
            $(
                m.insert($key, $value);
            )+
                m
        }
    };
);

lazy_static! {
    pub static ref TEMPLATE_MAP: TemplateMap = template_map! {
        "/" => "index",
        "404" => "404",
        "500" => "500",
        "/blog" => "blog/blog_root",
        "/linkedin" => "linkedin",
        "/github" => "github",
        "/resume_pdf" => "resume/dpbriggs_resume.pdf",
        "/resume" => "resume"
    };
}

pub fn get_template(uri: &str) -> &str {
    TEMPLATE_MAP.get(uri).unwrap()
}
