use crate::error::SiteError;
use chrono::NaiveDate;
use miette::Result;
use serde::Serialize;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Debug, Clone)]
pub struct Photo {
    pub filename: String,
    pub caption: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct PicSession {
    pub date: NaiveDate,
    pub date_str: String,
    pub title: String,
    pub description: String,
    pub photos: Vec<Photo>,
}

#[derive(Serialize, Debug)]
pub struct PicsGallery {
    pub sessions: Vec<PicSession>,
}

fn parse_pic_md(contents: &str) -> (String, String, Vec<Photo>) {
    let mut title = String::new();
    let mut description = String::new();
    let mut photos = Vec::new();
    let mut past_header = false;

    for line in contents.lines() {
        if !past_header {
            if line.trim().is_empty() {
                past_header = true;
                continue;
            }
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                match key {
                    "title" => title = value.to_owned(),
                    "description" => description = value.to_owned(),
                    _ => {}
                }
            }
        } else {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            // Check if this is a photo entry (has a known image extension)
            let (filename_part, caption_part) = if let Some(pos) = line.find(':') {
                let fname = line[..pos].trim();
                let cap = line[pos + 1..].trim();
                (fname, if cap.is_empty() { None } else { Some(cap.to_owned()) })
            } else {
                (line, None)
            };
            let ext = std::path::Path::new(filename_part)
                .extension()
                .and_then(OsStr::to_str)
                .unwrap_or("")
                .to_lowercase();
            if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif" | "avif") {
                photos.push(Photo {
                    filename: filename_part.to_owned(),
                    caption: caption_part,
                });
            }
        }
    }

    (title, description, photos)
}

pub fn get_pics_gallery(pics_root: &str) -> Result<PicsGallery> {
    let base = PathBuf::from(pics_root);
    if !base.is_dir() {
        // No pics directory yet — return empty gallery
        return Ok(PicsGallery { sessions: vec![] });
    }

    let mut sessions = Vec::new();

    for entry in fs::read_dir(&base).map_err(SiteError::from)? {
        let entry = entry.map_err(SiteError::from)?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let dir_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("")
            .to_owned();

        let date = match NaiveDate::parse_from_str(&dir_name, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => continue, // skip dirs that aren't YYYY-MM-DD
        };

        let pic_md_path = path.join("PIC.md");
        if !pic_md_path.exists() {
            eprintln!("Warning: no PIC.md found in {:?}, skipping", path);
            continue;
        }

        let contents = fs::read_to_string(&pic_md_path).map_err(SiteError::from)?;
        let (title, description, photos) = parse_pic_md(&contents);

        println!("Successfully parsed pics session {:?}", pic_md_path);

        sessions.push(PicSession {
            date,
            date_str: dir_name,
            title,
            description,
            photos,
        });
    }

    sessions.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(PicsGallery { sessions })
}
