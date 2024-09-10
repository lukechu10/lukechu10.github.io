use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

#[component]
fn Header() -> View {
    view! {
        header(class="p-2 border-b-2 border-gray-200") {
            nav(class="flex flex-row justify-between items-center") {
                div(class="self-start") {
                    a(href="/") { "lukechu" }
                }
                div(class="self-end flex flex-row gap-4") {
                    div { a(href="/about") { "About" } }
                    div { a(href="https://github.com/lukechu10") { "GitHub" } }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Route)]
enum Routes {
    #[to("/")]
    Home,
    #[to("/about")]
    About,
    #[not_found]
    NotFound,
}

#[component]
fn Route(route: ReadSignal<Routes>) -> View {
    view! {
        Header()
        (match route.get_clone() {
            Routes::Home => view! {
                main(class="p-4") {
                    h1 { "Home" }
                    p { "Welcome to my website!" }
                }
            },
            Routes::About => view! {
                main(class="p-4") {
                    h1 { "About" }
                    p { "I am a software developer." }
                }
            },
            Routes::NotFound => view! {
                main(class="p-4") {
                    h1 { "404 Not Found" }
                    p { "The page you are looking for does not exist." }
                }
            },
        })
    }
}

#[component]
fn App() -> View {
    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=Route
        )
    }
}

fn main() {
    sycamore::render(App);
}
