use chrono::NaiveDate;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::PathBuf;

type Slug = String;

/// OrgBlog represents all blog related items.
/// See [OrgBlog](crate::blog::OrgBlog) and
/// [get_org_blog](crate::blog::get_org_blog).
///
/// # Example
///
/// let org_blog: OrgBlog = get_org_blog()
#[derive(Serialize, Debug)]
pub struct OrgBlog {
    pub html: HashMap<Slug, OrgModeHtml>,
    // Blog files should be sorted by date (newest is at head)
    pub blog_files: Vec<OrgModeHtml>,
}

/// OrgModeHtml represents a particular org-mode blog article.
#[derive(Serialize, Debug, Clone)]
pub struct OrgModeHtml {
    pub title: String,
    pub date: NaiveDate,
    pub pub_date: String,
    pub toc: String,
    pub desc: String,
    pub html: String,
    pub slug: String,
    pub footnotes: Vec<String>,
}

fn get_html_files(base: &str) -> Result<Vec<PathBuf>, io::Error> {
    let base = PathBuf::from(base);
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
    Ok(html_files)
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    CannotFindToc(PathBuf),
    CannotParseDate(PathBuf),
    CannotFindTitle(PathBuf),
    CannotFindFirstParagraph(PathBuf),
    CannotFindDate(PathBuf),
    CannotFindContents(PathBuf),
    CannotParseHtml(PathBuf),
    CannotMakeSlug(PathBuf),
}

pub fn get_html_contents(blog_file: &PathBuf) -> Result<OrgModeHtml, ParsingError> {
    let file_contents = fs::read_to_string(&blog_file);
    let document = match file_contents {
        Ok(contents) => Document::from(&contents[..]),
        Err(e) => {
            println!("{:?}", e);
            return Err(ParsingError::CannotParseHtml(blog_file.to_path_buf()));
        }
    };

    let title = match document.find(Class("title")).next() {
        Some(t) => t.text(),
        None => return Err(ParsingError::CannotFindTitle(blog_file.to_path_buf())),
    };

    let date_string = match document.find(Class("timestamp")).next() {
        Some(date) => date.text(),
        None => return Err(ParsingError::CannotFindDate(blog_file.to_path_buf())),
    };

    // <2019-02-06 Wed> == <%Y-%m-%d %a>
    let date = match NaiveDate::parse_from_str(&date_string[..], "<%Y-%m-%d %a>") {
        Ok(d) => d,
        Err(e) => {
            error!("Could not parse date for {:?}, reason {:?}", date_string, e);
            return Err(ParsingError::CannotParseDate(blog_file.to_path_buf()));
        }
    };

    let pub_date: String = date.format("%a, %d %b %Y 1:01:00 EST").to_string();

    let toc = match document.find(Attr("id", "text-table-of-contents")).next() {
        Some(toc) => toc.html(),
        None => return Err(ParsingError::CannotFindToc(blog_file.to_path_buf())),
    };

    let html = match document.find(Class("outline-2")).next() {
        Some(org_body) => org_body,
        None => return Err(ParsingError::CannotFindContents(blog_file.to_path_buf())),
    };

    // The first paragraph is likely a good enough description.
    let desc = match html.find(Name("p")).next() {
        Some(first_para) => first_para.text(),
        None => {
            return Err(ParsingError::CannotFindFirstParagraph(
                blog_file.to_path_buf(),
            ))
        }
    };

    let slug_string = blog_file.clone().into_os_string().into_string().unwrap();

    let slug = match slug_string.split('/').last() {
        Some(s) => s.replace(".html", ""),
        None => return Err(ParsingError::CannotMakeSlug(blog_file.to_path_buf())),
    };

    let footnotes = document.find(Class("footdef")).map(|x| x.html()).collect();

    info!("Successfully parsed {:?}", blog_file);

    Ok(OrgModeHtml {
        title,
        date,
        pub_date,
        toc,
        desc,
        html: html.html(),
        slug,
        footnotes,
    })
}

pub fn get_org_mode_files(blog_root: &str) -> Vec<OrgModeHtml> {
    match get_html_files(blog_root) {
        Ok(org) => {
            let html_res: Vec<_> = org.iter().map(|o| get_html_contents(&o)).collect();
            let mut html_success: Vec<OrgModeHtml> = Vec::new();
            for html in html_res {
                match html {
                    Ok(h) => html_success.push(h),
                    Err(e) => error!("Failed to parse file {:?}", e),
                }
            }
            html_success.sort_by(|a, b| b.date.cmp(&a.date));
            html_success
        }
        Err(e) => {
            error!("Cannot get org mode files!");
            panic!("{}", e);
        }
    }
}

pub fn get_org_blog(blog_root: &str) -> OrgBlog {
    let blog_files = get_org_mode_files(blog_root);
    let html: HashMap<Slug, OrgModeHtml> = blog_files
        .clone()
        .into_iter()
        .map(|x| (x.slug.clone(), x))
        .collect();
    OrgBlog { html, blog_files }
}
