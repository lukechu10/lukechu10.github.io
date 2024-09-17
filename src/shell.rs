use sycamore::prelude::*;

use crate::Routes;

#[component]
pub fn Route(route: ReadSignal<Routes>) -> View {
    view! {
        div(class="app flex flex-col min-h-screen text-slate-200 bg-slate-900") {
            Header()
            main(class="mt-6 flex-grow") {
                (match route.get_clone() {
                    Routes::Home => view! {
                        crate::pages::home::Home()
                    },
                    Routes::About => view! {
                        crate::pages::about::About()
                    },
                    Routes::Post(id) => view! {
                        crate::pages::post::PostView(id=id)
                    },
                    Routes::NotFound => view! {
                        NotFound()
                    },
                })
            }
            Footer()
        }
    }
}

#[component]
fn Header() -> View {
    view! {
        header(class="p-2 border-b-2 border-slate-500 font-mono") {
            nav(class="flex flex-row justify-between items-center") {
                div(class="self-start") {
                    a(class="hover:underline font-bold", href="/") { "$ cd /home/lukechu" }
                }
                div(class="self-end flex flex-row gap-4") {
                    div { a(class="hover:underline", href="/about") { "about" } }
                    div { a(class="hover:underline", href="https://github.com/lukechu10") { "github" } }
                }
            }
        }
    }
}

#[component]
fn Footer() -> View {
    view! {
        footer(class="p-2 border-t-2 border-slate-500 text-xs font-mono") {
            div(class="flex flex-row justify-between") {
                div { "© 2024 Luke Chu" }
                div {
                    "Made with "
                    a(class="hover:underline", href="https://rust-lang.org") { "Rust" }
                    " and "
                    a(class="hover:underline", href="https://github.com/sycamore-rs/sycamore") { "Sycamore" }
                }
            }
        }
    }
}

#[component]
pub fn NotFound() -> View {
    view! {
        h1 { "404 Not Found" }
        p { "The page you are looking for does not exist." }
    }
}
