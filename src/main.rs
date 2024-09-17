mod pages;
mod components;
mod shell;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

#[derive(Debug, Clone, Route)]
enum Routes {
    #[to("/")]
    Home,
    #[to("/about")]
    About,
    #[to("/post/<id>")]
    Post(String),
    #[not_found]
    NotFound,
}

#[component]
fn App() -> View {
    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=shell::Route,
        )
    }
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
