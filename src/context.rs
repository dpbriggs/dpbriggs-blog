use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::blog::{get_org_mode_files, OrgModeHtml};

pub type SiteContextKv = HashMap<String, String>;
pub type SiteContextBlog = HashMap<String, String>;
type TemplateMap = HashMap<&'static str, &'static str>;

#[derive(Serialize, Debug)]
pub struct RealSiteContext {
    pub base: &'static SiteContextKv,
    pub kv: SiteContextKv,
    // blog: &'static OrgModeHtml,
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

pub fn get_base_context() -> SiteContextKv {
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

pub fn get_special_context() -> RealSiteContext {
    RealSiteContext {
        base: &STATIC_SITE_CONTEXT_KV,
        kv: SiteContextKv::new(),
    }
}

lazy_static! {
    pub static ref TEMPLATE_MAP: TemplateMap = template_map! {
        "/" => "index",
        "404" => "404",
        "500" => "500",
        "/blog" => "blog/blog_root",
        "/linkedin" => "linkedin",
        "/github" => "github",
        "/resume_pdf" => "resume/dpbriggs_resume.pdf",
        "/resume" => "resume",
        "/blog/<article>" => "blog/article"
    };
}

pub fn get_template(uri: &str) -> &str {
    TEMPLATE_MAP.get(uri).unwrap()
}
