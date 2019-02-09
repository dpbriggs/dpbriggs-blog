#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::Client;

    fn get_client() -> Client {
        use crate::server::start_server;
        Client::new(start_server()).expect("Failed to start_server!")
    }

    #[test]
    fn static_pages_should_200() {
        let routes = vec![
            "/",
            "/github",
            "/linkedin",
            "/resume",
            "/blog",
            "/resume_pdf",
            "/static/favicons/apple-touch-icon.png",
            "/static/favicons/favicon-32x32.png",
            "/static/favicons/favicon-16x16.png",
            "/static/favicons/site.webmanifest",
            "/static/favicons/safari-pinned-tab.svg",
            "/static/favicons/favicon.ico",
            "/static/favicons/browserconfig.xml",
            "/static/css/dpbriggs.css",
            "/static/css/solarized-dark.css",
            "/static/css/bootstrap-slate.css",
        ];
        let client = get_client();
        for route in routes {
            let resp = client.get(route).dispatch();
            assert_eq!(resp.status(), Status::Ok);
        }
    }

    #[test]
    fn unknown_pages_should_404() {
        let routes = vec!["/adawdawd", "/ada2wdawd/adawdw", "/ada3dawd/afwf/afwafa"];
        let client = get_client();
        for route in routes {
            let resp = client.get(route).dispatch();
            println!("{}", route);
            assert_eq!(resp.status(), Status::NotFound);
        }
    }

    #[test]
    fn five_hundred_page_should_500() {
        let client = get_client();
        let resp = client.get("/500").dispatch();
        assert_eq!(resp.status(), Status::InternalServerError);
    }

    #[test]
    fn base_context_should_contain_necessary_keys() {
        use crate::context::get_base_context;
        let base_context = get_base_context("/").base;
        let necessary_keys = vec![
            "domain_name",
            "nav_site_href",
            "root_uri",
            "blog_uri",
            "resume_uri",
            "linkedin_uri",
            "github_uri",
            "resume_uri",
            "resume_pdf_uri",
            "crash_uri",
            "web_sep",
            "admin_email",
            "full_name",
            "internet_handle",
            "my_email",
            "github_url",
            "github_repo_url",
            "linkedin_url",
            "nav_site_href",
        ];
        for key in necessary_keys {
            assert!(base_context.contains_key(key))
        }
    }

    #[test]
    fn templates_should_exist() {
        use crate::context::TEMPLATE_MAP;
        let templates = vec![
            "/",
            "404",
            "500",
            "/blog",
            "/linkedin",
            "/github",
            "/resume_pdf",
            "/resume",
        ];
        for template in templates {
            assert!(TEMPLATE_MAP.contains_key(template))
        }
    }

    #[test]
    fn blog_files_should_be_parsable() {
        use crate::blog::get_org_mode_files;
        use crate::context::BLOG_ROOT;
        get_org_mode_files(BLOG_ROOT);
    }

    #[test]
    fn org_mode_files_should_have_matching_html() {
        use crate::context::BLOG_ROOT;
        use std::ffi::OsStr;
        use std::fs;
        use std::path::PathBuf;

        let base = PathBuf::from(BLOG_ROOT);
        if !base.is_dir() {
            panic!("BLOG_ROOT is not a directory!")
        }

        for entry in fs::read_dir(base).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let mut num_org = 0;
            let mut num_html = 0;
            for file in fs::read_dir(path).unwrap() {
                let file = file.unwrap();
                let path = file.path();
                if path.is_dir() {
                    continue;
                }
                let extension = path.extension().and_then(OsStr::to_str).unwrap_or("");
                match extension {
                    "html" => num_html += 1,
                    "org" => num_org += 1,
                    _ => println!("Unknown file extensions {} found!", extension),
                }
            }
            assert_eq!(num_html, num_org);
        }
    }

    #[test]
    fn org_parser_should_parse_good_files() {
        use crate::blog::get_html_contents;
        use std::path::PathBuf;
        let test_blog_path = PathBuf::from("tests/good-blog-files/2420-04-20/good-blog-file.html");
        let res = get_html_contents(&test_blog_path);
        dbg!(&res);
        assert!(res.is_ok());
    }

    #[test]
    fn org_parser_should_throw_applicable_errors() {
        use crate::blog::{get_html_contents, ParsingError};
        use std::path::PathBuf;
        let missing_date_loc = "tests/bad-org-mode-files/missing-date.html";
        let missing_date = PathBuf::from(missing_date_loc);
        let missing_date_err = ParsingError::CannotFindDate(missing_date.clone());

        let bad_html_loc = "tests/bad-org-mode-files/non-existent.html";
        let bad_html = PathBuf::from(bad_html_loc);
        let bad_html_err = ParsingError::CannotParseHtml(bad_html.clone());

        let missing_title_loc = "tests/bad-org-mode-files/missing-title.html";
        let missing_title = PathBuf::from(missing_title_loc);
        let missing_title_err = ParsingError::CannotFindTitle(missing_title.clone());

        let missing_toc_loc = "tests/bad-org-mode-files/missing-toc.html";
        let missing_toc = PathBuf::from(missing_toc_loc);
        let missing_toc_err = ParsingError::CannotFindToc(missing_toc.clone());

        match get_html_contents(&missing_date) {
            Ok(_) => panic!("Successfully parsed bad file {:?}", missing_date_loc),
            Err(e) => assert_eq!(e, missing_date_err),
        }
        match get_html_contents(&bad_html) {
            Ok(_) => panic!("Successfully parsed bad file {:?}", bad_html_loc),
            Err(e) => assert_eq!(e, bad_html_err),
        }
        match get_html_contents(&missing_title) {
            Ok(_) => panic!("Successfully parsed bad file {:?}", missing_title_loc),
            Err(e) => assert_eq!(e, missing_title_err),
        }
        match get_html_contents(&missing_toc) {
            Ok(_) => panic!("Successfully parsed bad file {:?}", missing_toc_loc),
            Err(e) => assert_eq!(e, missing_toc_err),
        }

        // let test_blog_path = PathBuf::from("tests/good-blog-files/2420-04-20/good-blog-file.html");
        // let res = get_html_contents(&test_blog_path);
    }

}
