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
    pub title: Option<String>,
    pub maps_url: Option<String>,
    pub description: Option<String>,
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

fn render_md_links(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut remaining = text;
    while let Some(bracket_start) = remaining.find('[') {
        result.push_str(&remaining[..bracket_start]);
        remaining = &remaining[bracket_start + 1..];
        if let Some(bracket_end) = remaining.find(']') {
            let link_text = &remaining[..bracket_end];
            let after_bracket = &remaining[bracket_end + 1..];
            if after_bracket.starts_with('(') {
                if let Some(paren_end) = after_bracket.find(')') {
                    let url = &after_bracket[1..paren_end];
                    result.push_str(&format!(
                        "<a href=\"{url}\" target=\"_blank\">{link_text}</a>"
                    ));
                    remaining = &after_bracket[paren_end + 1..];
                    continue;
                }
            }
            // Not a valid link — emit literally
            result.push('[');
            result.push_str(link_text);
            result.push(']');
            remaining = after_bracket;
        } else {
            result.push('[');
        }
    }
    result.push_str(remaining);
    result
}

fn is_image_ext(path: &str) -> bool {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
        .to_lowercase();
    matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif" | "avif")
}

fn parse_pic_md(contents: &str) -> (String, String, Vec<Photo>) {
    let mut session_title = String::new();
    let mut session_description_lines: Vec<&str> = Vec::new();
    let mut photos: Vec<Photo> = Vec::new();

    // Current photo block being built
    let mut current_title: Option<String> = None;
    let mut current_filename: Option<String> = None;
    let mut current_maps: Option<String> = None;
    let mut current_desc_lines: Vec<&str> = Vec::new();
    let mut in_photo_block = false;

    let flush_photo = |photos: &mut Vec<Photo>,
                       current_title: &mut Option<String>,
                       current_filename: &mut Option<String>,
                       current_maps: &mut Option<String>,
                       current_desc_lines: &mut Vec<&str>| {
        if let Some(filename) = current_filename.take() {
            let description = {
                let s = current_desc_lines.join("\n").trim().to_owned();
                if s.is_empty() { None } else { Some(render_md_links(&s)) }
            };
            photos.push(Photo {
                filename,
                title: current_title.take().map(|t| render_md_links(&t)),
                maps_url: current_maps.take(),
                description,
            });
        } else {
            // No file: line yet — discard partial block
            current_title.take();
            current_maps.take();
        }
        current_desc_lines.clear();
    };

    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix("# ") {
            // Session title
            session_title = rest.trim().to_owned();
        } else if let Some(rest) = line.strip_prefix("## ") {
            // Start of a new photo block — flush any previous
            if in_photo_block {
                flush_photo(
                    &mut photos,
                    &mut current_title,
                    &mut current_filename,
                    &mut current_maps,
                    &mut current_desc_lines,
                );
            }
            in_photo_block = true;
            current_title = Some(rest.trim().to_owned());
            current_filename = None;
            current_maps = None;
            current_desc_lines.clear();
        } else if in_photo_block {
            if let Some(rest) = line.strip_prefix("file:") {
                current_filename = Some(rest.trim().to_owned());
            } else if let Some(rest) = line.strip_prefix("maps:") {
                let url = rest.trim();
                if !url.is_empty() {
                    current_maps = Some(url.to_owned());
                }
            } else if is_image_ext(line.trim()) {
                // Bare filename with no key prefix
                current_filename = Some(line.trim().to_owned());
            } else if !line.trim().is_empty() {
                current_desc_lines.push(line);
            }
        } else {
            // Between session title and first photo block — session description
            if !line.starts_with('#') {
                session_description_lines.push(line);
            }
        }
    }

    if in_photo_block {
        flush_photo(
            &mut photos,
            &mut current_title,
            &mut current_filename,
            &mut current_maps,
            &mut current_desc_lines,
        );
    }

    let session_description = render_md_links(
        session_description_lines.join("\n").trim()
    );

    (session_title, session_description, photos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn md_links_converts_link() {
        assert_eq!(
            render_md_links("see [Foo](https://example.com) here"),
            "see <a href=\"https://example.com\" target=\"_blank\">Foo</a> here"
        );
    }

    #[test]
    fn md_links_multiple_links() {
        let out = render_md_links("[A](https://a.com) and [B](https://b.com)");
        assert!(out.contains("<a href=\"https://a.com\""));
        assert!(out.contains("<a href=\"https://b.com\""));
    }

    #[test]
    fn md_links_no_links_unchanged() {
        assert_eq!(render_md_links("plain text"), "plain text");
    }

    #[test]
    fn md_links_malformed_not_converted() {
        // Missing closing paren — emitted literally
        assert_eq!(render_md_links("[text](no-close"), "[text](no-close");
        // No parens after bracket — emitted literally
        assert_eq!(render_md_links("[text] stuff"), "[text] stuff");
    }

    #[test]
    fn parses_session_title() {
        let (title, _, _) = parse_pic_md("# My Trip\n");
        assert_eq!(title, "My Trip");
    }

    #[test]
    fn parses_session_description() {
        let (_, desc, _) = parse_pic_md("# Title\n\nA lovely day out.\n\n## Photo\nfile: a.jpg\n");
        assert_eq!(desc, "A lovely day out.");
    }

    #[test]
    fn parses_photo_title_and_filename() {
        let (_, _, photos) = parse_pic_md("# Title\n\n## Golden Gate\nfile: gate.jpg\n");
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].filename, "gate.jpg");
        assert_eq!(photos[0].title.as_deref(), Some("Golden Gate"));
    }

    #[test]
    fn parses_maps_url() {
        let input = "# T\n\n## P\nfile: x.jpg\nmaps: https://maps.google.com/?q=foo\n";
        let (_, _, photos) = parse_pic_md(input);
        assert_eq!(photos[0].maps_url.as_deref(), Some("https://maps.google.com/?q=foo"));
    }

    #[test]
    fn parses_photo_description() {
        let input = "# T\n\n## P\nfile: x.jpg\n\nThis is a nice spot.\n";
        let (_, _, photos) = parse_pic_md(input);
        assert_eq!(photos[0].description.as_deref(), Some("This is a nice spot."));
    }

    #[test]
    fn optional_fields_are_none_when_absent() {
        let (_, _, photos) = parse_pic_md("# T\n\n## P\nfile: x.jpg\n");
        assert!(photos[0].maps_url.is_none());
        assert!(photos[0].description.is_none());
    }

    #[test]
    fn parses_multiple_photos() {
        let input = "# T\n\n## First\nfile: a.jpg\n\n## Second\nfile: b.jpg\n";
        let (_, _, photos) = parse_pic_md(input);
        assert_eq!(photos.len(), 2);
        assert_eq!(photos[0].filename, "a.jpg");
        assert_eq!(photos[1].filename, "b.jpg");
    }

    #[test]
    fn bare_filename_without_file_prefix() {
        let (_, _, photos) = parse_pic_md("# T\n\n## P\na.jpg\n");
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].filename, "a.jpg");
    }

    #[test]
    fn photo_block_without_file_is_skipped() {
        let (_, _, photos) = parse_pic_md("# T\n\n## P\nmaps: https://example.com\n");
        assert_eq!(photos.len(), 0);
    }

    #[test]
    fn empty_input_gives_empty_results() {
        let (title, desc, photos) = parse_pic_md("");
        assert_eq!(title, "");
        assert_eq!(desc, "");
        assert!(photos.is_empty());
    }

    #[test]
    fn parses_all_supported_image_extensions() {
        for ext in &["jpg", "jpeg", "png", "webp", "gif", "avif"] {
            let input = format!("# T\n\n## P\nfile: photo.{ext}\n");
            let (_, _, photos) = parse_pic_md(&input);
            assert_eq!(photos.len(), 1, "failed for extension: {ext}");
        }
    }

    #[test]
    fn parses_real_world_example() {
        let input = r#"# First Roll

Testing out the new camera.

## First shot of the day
file: Botanical.jpg
maps: https://maps.google.com/?q=botanical+garden

Beautiful light that morning.

## Trying out filters
file: VictoriaParkBuilding.jpg
"#;
        let (title, desc, photos) = parse_pic_md(input);
        assert_eq!(title, "First Roll");
        assert_eq!(desc, "Testing out the new camera.");
        assert_eq!(photos.len(), 2);
        assert_eq!(photos[0].filename, "Botanical.jpg");
        assert_eq!(photos[0].title.as_deref(), Some("First shot of the day"));
        assert!(photos[0].maps_url.is_some());
        assert_eq!(photos[0].description.as_deref(), Some("Beautiful light that morning."));
        assert_eq!(photos[1].filename, "VictoriaParkBuilding.jpg");
        assert!(photos[1].maps_url.is_none());
    }
}

pub fn get_pics_gallery(pics_root: &str) -> Result<PicsGallery> {
    let base = PathBuf::from(pics_root);
    if !base.is_dir() {
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
            Err(_) => continue,
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
