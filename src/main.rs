mod components;
mod error_pages;
mod templates;

use perseus::{Html, PerseusApp};

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(templates::index::get_template)
        .template(templates::about::get_template)
        .error_pages(error_pages::get_error_pages)
}
