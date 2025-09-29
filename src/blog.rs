use crate::error::SiteError;
use chrono::NaiveDate;
use miette::{Diagnostic, Result};
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use serde::Serialize;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

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

fn get_html_files(base: &str) -> Result<Vec<PathBuf>> {
    let base = PathBuf::from(base);
    if !base.is_dir() {
        return Err(SiteError::NotADirectory(base).into());
    }
    let mut html_files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&base).map_err(SiteError::from)? {
        let entry = entry.map_err(SiteError::from)?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        for file in fs::read_dir(path).map_err(SiteError::from)? {
            let file = file.map_err(SiteError::from)?;
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

#[derive(Debug, Error, Diagnostic, PartialEq)]
pub enum ParsingError {
    #[error("Cannot find TOC in {0}")]
    #[diagnostic(code(app::parsing::cannot_find_toc))]
    CannotFindToc(PathBuf),
    #[error("Cannot parse date in {0}")]
    #[diagnostic(code(app::parsing::cannot_parse_date))]
    CannotParseDate(PathBuf),
    #[error("Cannot find title in {0}")]
    #[diagnostic(code(app::parsing::cannot_find_title))]
    CannotFindTitle(PathBuf),
    #[error("Cannot find first paragraph in {0}")]
    #[diagnostic(code(app::parsing::cannot_find_first_paragraph))]
    CannotFindFirstParagraph(PathBuf),
    #[error("Cannot find date in {0}")]
    #[diagnostic(code(app::parsing::cannot_find_date))]
    CannotFindDate(PathBuf),
    #[error("Cannot find contents in {0}")]
    #[diagnostic(code(app::parsing::cannot_find_contents))]
    CannotFindContents(PathBuf),
    #[error("Cannot make slug for {0}")]
    #[diagnostic(code(app::parsing::cannot_make_slug))]
    CannotMakeSlug(PathBuf),
}

pub fn get_html_contents(blog_file: &Path) -> Result<OrgModeHtml> {
    let file_contents = fs::read_to_string(blog_file).map_err(SiteError::from)?;
    let document = Document::from(file_contents.as_str());

    let title = document
        .find(Class("title"))
        .next()
        .ok_or_else(|| ParsingError::CannotFindTitle(blog_file.to_path_buf()))?;

    let date_string = document
        .find(Class("timestamp"))
        .next()
        .ok_or_else(|| ParsingError::CannotFindDate(blog_file.to_path_buf()))?;

    // <2019-02-06 Wed> == <%Y-%m-%d %a>
    let date = NaiveDate::parse_from_str(date_string.text().as_str(), "<%Y-%m-%d %a>")
        .map_err(|_| SiteError::from(ParsingError::CannotParseDate(blog_file.to_path_buf())))?;

    let pub_date: String = date.format("%a, %d %b %Y 1:01:00 EST").to_string();

    let toc = document
        .find(Attr("id", "text-table-of-contents"))
        .next()
        .ok_or_else(|| ParsingError::CannotFindToc(blog_file.to_path_buf()))?;

    let html = document
        .find(Class("outline-2"))
        .next()
        .ok_or_else(|| ParsingError::CannotFindContents(blog_file.to_path_buf()))?;

    // The first paragraph is likely a good enough description.
    let desc = html
        .find(Name("p"))
        .next()
        .ok_or_else(|| ParsingError::CannotFindFirstParagraph(blog_file.to_path_buf()))?;

    let slug_string = blog_file.to_string_lossy();

    let slug = slug_string
        .split('/')
        .next_back()
        .ok_or_else(|| ParsingError::CannotMakeSlug(blog_file.to_path_buf()))?
        .replace(".html", "");

    let footnotes = document.find(Class("footdef")).map(|x| x.html()).collect();

    println!("Successfully parsed {:?}", blog_file);

    Ok(OrgModeHtml {
        title: title.text(),
        date,
        pub_date,
        toc: toc.html(),
        desc: desc.text(),
        html: html.html(),
        slug,
        footnotes,
    })
}

pub fn get_org_mode_files(blog_root: &str) -> Result<Vec<OrgModeHtml>> {
    let org_files = get_html_files(blog_root)?;
    let mut html_success: Vec<OrgModeHtml> = Vec::new();
    for html_file in org_files {
        match get_html_contents(&html_file) {
            Ok(h) => html_success.push(h),
            Err(e) => eprintln!("Failed to parse file {:?}: {}", html_file, e),
        }
    }
    html_success.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(html_success)
}

pub fn get_org_blog(blog_root: &str) -> Result<OrgBlog> {
    let blog_files = get_org_mode_files(blog_root)?;
    let html: HashMap<Slug, OrgModeHtml> = blog_files
        .clone()
        .into_iter()
        .map(|x| (x.slug.clone(), x))
        .collect();
    Ok(OrgBlog { html, blog_files })
}
