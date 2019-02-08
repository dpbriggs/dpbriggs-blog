extern crate chrono;
extern crate select;
use chrono::NaiveDate;
use select::document::Document;
use select::predicate::{Attr, Class};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::PathBuf;

type Slug = String;
static BLOG_ROOT: &'static str = "blog/";

#[derive(Serialize, Debug)]
pub struct OrgBlog {
    pub html: HashMap<Slug, OrgModeHtml>,
    // Blog files should be sorted by date (newest is at head)
    pub blog_files: Vec<OrgModeHtml>,
}

#[derive(Serialize, Debug, Clone)]
pub struct OrgModeHtml {
    pub title: String,
    pub date: NaiveDate,
    pub toc: String,
    pub html: String,
    pub blog_string: String,
    pub slug: String,
}

fn get_html_files() -> Result<Vec<PathBuf>, io::Error> {
    let base = PathBuf::from(BLOG_ROOT);
    if !base.is_dir() {
        panic!("BLOG_ROOT is not a directory!")
    }
    let mut html_files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(base)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        for file in fs::read_dir(path)? {
            let file = file?;
            let path = file.path();
            if path.is_dir() {
                continue;
            }
            if path.extension().and_then(OsStr::to_str).unwrap_or("") == "html" {
                html_files.push(path);
            }
        }
    }
    dbg!(&html_files);
    Ok(html_files)
}

fn get_html_contents(blog_files: Vec<PathBuf>) -> Vec<OrgModeHtml> {
    let mut org_mode_files = Vec::new();

    for blog_file in blog_files {
        let file_contents = fs::read_to_string(&blog_file);
        let document = match file_contents {
            Ok(contents) => Document::from(&contents[..]),
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };
        let title = document.find(Class("title")).next().unwrap().text();
        let date_string = document.find(Class("timestamp")).next().unwrap().text();
        // <2019-02-06 Wed> == <%Y-%m-%d %a>
        let date = match NaiveDate::parse_from_str(&date_string[..], "<%Y-%m-%d %a>") {
            Ok(d) => d,
            Err(e) => {
                error!("Could not parse date for {:?}", date_string);
                panic!("Failed to parse date! {:?}", e)
            }
        };

        let toc = document
            .find(Attr("id", "text-table-of-contents"))
            .next()
            .unwrap()
            .html();
        let html = document.find(Class("outline-2")).next().unwrap().html();
        let blog_string = document.find(Class("outline-2")).next().unwrap().text();
        dbg!(&blog_string);
        let slug = blog_file
            .into_os_string()
            .into_string()
            .unwrap()
            .split("/")
            .last()
            .unwrap()
            .replace(".html", "");
        org_mode_files.push(OrgModeHtml {
            title,
            date,
            toc,
            html,
            blog_string,
            slug,
        })
    }

    dbg!(&org_mode_files);
    org_mode_files.sort_by(|a, b| a.date.cmp(&b.date));
    org_mode_files
}

pub fn get_org_mode_files() -> Vec<OrgModeHtml> {
    match get_html_files() {
        Ok(org) => get_html_contents(org),
        Err(e) => panic!(e),
    }
}

pub fn get_org_blog() -> OrgBlog {
    let blog_files = get_org_mode_files();
    let html: HashMap<Slug, OrgModeHtml> = blog_files
        .clone()
        .into_iter()
        .map(|x| (x.slug.clone(), x))
        .collect();
    OrgBlog { html, blog_files }
}
