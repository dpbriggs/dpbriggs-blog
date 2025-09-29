extern crate log;

#[cfg(test)]
mod tests;

mod blog;
mod context;
mod error;
mod routes;

use clap::Parser;
use fs_extra::dir::{self, CopyOptions};
use miette::Result;
use std::fs;
use std::path::Path;
use tera::Tera;

use crate::blog::get_org_blog;
use crate::context::BLOG_ROOT;
use crate::error::SiteError;
use crate::routes::generate_site;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Extra paths to copy to the output directory
    #[arg(long, num_args = 1..)]
    extra_paths: Vec<String>,
    /// Output directory.
    #[arg(long, default_value = "public")]
    output_dir: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let tera = Tera::new("templates/**/*.tera").map_err(SiteError::from)?;

    // Create the output directory
    let output_dir = &cli.output_dir;
    println!("Creating output directory: {}", output_dir);
    if fs::metadata(output_dir).is_ok() {
        fs::remove_dir_all(output_dir).map_err(SiteError::from)?;
    }
    fs::create_dir(output_dir).map_err(SiteError::from)?;

    // Copy static files
    let static_dir = "static";
    println!("Copying static files from: {}", static_dir);
    let output_static_dir = format!("{}/{}", output_dir, static_dir);
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    copy_options.content_only = true;
    dir::copy(static_dir, &output_static_dir, &copy_options).map_err(SiteError::from)?;

    // Copy extra paths
    for path_str in &cli.extra_paths {
        let path = Path::new(path_str);
        println!("Copying extra path: {:?}", path);
        let file_name = path
            .file_name()
            .ok_or_else(|| SiteError::FileNotFound(path_str.clone()))?;
        let dest_path = Path::new(output_dir).join(file_name);
        if path.is_dir() {
            dir::copy(path, dest_path, &copy_options).map_err(SiteError::from)?;
        } else {
            fs::copy(path, dest_path).map_err(SiteError::from)?;
        }
    }

    let blog = get_org_blog(BLOG_ROOT)?;

    println!("Generating site...");
    generate_site(&tera, output_dir, &blog)?;
    println!("Site generation complete.");

    Ok(())
}
