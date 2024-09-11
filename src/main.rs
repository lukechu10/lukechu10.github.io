mod pages;
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
fn Route(route: ReadSignal<Routes>) -> View {
    view! {
        shell::Header()
        main(class="p-4 mb-auto") {
            (match route.get_clone() {
                Routes::Home => view! {
                    pages::home::Home()
                },
                Routes::About => view! {
                    h1 { "About" }
                    p { "I am a software developer." }
                },
                Routes::Post(id) => view! {
                    pages::post::PostView(id=id)
                },
                Routes::NotFound => view! {
                    NotFound()
                },
            })
        }
        shell::Footer()
    }
}

#[component]
fn NotFound() -> View {
    view! {
        h1 { "404 Not Found" }
        p { "The page you are looking for does not exist." }
    }
}

#[component]
fn App() -> View {
    view! {
        div(class="app flex flex-col h-screen justify-between bg-stone-50") {
            Router(
                integration=HistoryIntegration::new(),
                view=Route
            )
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
