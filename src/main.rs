mod components;
mod pages;
mod server_component;
mod shell;

use sycamore::prelude::*;
use sycamore_router::Route;

#[derive(Debug, Clone, PartialEq, Eq, Route)]
pub enum Routes {
    #[to("/")]
    Home,
    #[to("/about")]
    About,
    #[to("/post/<id>")]
    Post(String),
    #[not_found]
    NotFound,
}

#[cfg_not_ssr]
fn main() {
    use sycamore_router::{HistoryIntegration, Router};

    console_error_panic_hook::set_once();
    sycamore::hydrate_to(
        || {
            view! {
                shell::Shell {
                    Router(integration=HistoryIntegration::new(), view=shell::App)
                }
            }
        },
        &sycamore::web::document(),
    );
}

#[cfg_ssr]
#[tokio::main]
async fn main() {
    use std::{fs, path::PathBuf};

    static PUBLIC_PATH: &str = "dist/.stage";

    for (route, path) in get_static_paths() {
        let path = path.trim_start_matches('/');
        let path = PathBuf::from(PUBLIC_PATH).join(path);

        eprintln!("Rendering `{}`", path.display());

        let html = sycamore::render_to_string_await_suspense(|| {
            view! {
                shell::Shell {
                    sycamore_router::StaticRouter(route=route, view=shell::App)
                }
            }
        })
        .await;

        let dir = path.parent().expect("failed to get parent dir");
        fs::create_dir_all(dir).expect("failed to create parent dir");
        fs::write(path, format!("<!DOCTYPE html>{html}")).expect("failed to write html file");
    }

    let mut server_components = server_component::SERVER_COMPONENTS.lock().unwrap();
    for (id, html) in server_components.drain() {
        let path = PathBuf::from(PUBLIC_PATH)
            .join("server_components")
            .join(format!("{}.html", id));

        eprintln!("Rendering server component `{id}` to `{}`", path.display());

        let dir = path.parent().expect("failed to get parent dir");
        fs::create_dir_all(dir).expect("failed to create parent dir");
        fs::write(path, html).expect("failed to write html file");
    }

    eprintln!("Generating sitemap.xml");
    let sitemap = generate_sitemap_xml().expect("failed to generate sitemap");
    fs::write(PathBuf::from(PUBLIC_PATH).join("sitemap.xml"), sitemap)
        .expect("failed to write sitemap.xml");
}

pub fn get_static_paths() -> Vec<(Routes, String)> {
    let mut paths = vec![];

    paths.push((Routes::Home, "/index.html".to_string()));
    paths.push((Routes::About, "/about.html".to_string()));
    paths.push((Routes::NotFound, "/404.html".to_string()));

    for post in pages::post::POSTS.keys() {
        paths.push((Routes::Post(post.clone()), format!("/post/{post}.html")));
    }

    paths
}

/// Generate an XML sitemap file.
pub fn generate_sitemap_xml() -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    static BASE_URL: &str = "https://lukechu.dev";

    let paths = get_static_paths();
    let mut buf = String::new();

    write!(
        &mut buf,
        r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#
    )?;

    for (route, path) in paths {
        if route == Routes::NotFound {
            continue;
        }
        let path = path
            .strip_suffix(".html")
            .expect("should be an html page")
            .trim_end_matches("index");
        let loc = format!("{BASE_URL}{path}");

        write!(&mut buf, r#"<url><loc>{loc}</loc></url>"#)?;
    }

    write!(&mut buf, r#"</urlset>"#)?;

    Ok(buf)
}
