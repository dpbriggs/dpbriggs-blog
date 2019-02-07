extern crate select;
use select::document::Document;
use select::predicate::{Attr, Class};
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::PathBuf;

static BLOG_ROOT: &'static str = "blog/";

#[derive(Debug)]
pub struct OrgModeHtml {
    title: String,
    date: String,
    toc: String,
    blog_contents: String,
    file_path: String,
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
        let date = document.find(Class("timestamp")).next().unwrap().text();
        let toc = document
            .find(Attr("id", "table-of-contents"))
            .next()
            .unwrap()
            .html();
        let blog_contents = document.find(Class("outline-2")).next().unwrap().html();
        let file_path = blog_file.into_os_string().into_string().unwrap();
        org_mode_files.push(OrgModeHtml {
            title,
            date,
            toc,
            blog_contents,
            file_path,
        })
    }

    dbg!(&org_mode_files);
    org_mode_files
}

pub fn get_org_mode_files() -> Vec<OrgModeHtml> {
    match get_html_files() {
        Ok(org) => get_html_contents(org),
        Err(e) => panic!(e),
    }
}
