use sycamore::prelude::*;

use crate::Routes;

#[component]
pub fn Route(route: ReadSignal<Routes>) -> View {
    view! {
        div(class="app flex flex-col min-h-screen text-slate-200 bg-slate-950 text-base") {
            Header()
            main(class="mt-6 mx-3 flex-grow") {
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
        header(class="z-50 p-2 bg-inherit border-b-2 border-slate-700 text-sm font-mono text-red-200 sm:px-10 md:px-20 lg:px-40 xl:px-60") {
            nav(class="flex flex-row justify-between items-center") {
                div(class="self-start hover:underline font-bold") {
                    a(class="hidden sm:inline", href="/") { "$ cd /home/lukechu" }
                    a(class="inline sm:hidden", href="/") { "lukechu" }
                }
                div(class="self-end flex flex-row gap-4 sm:gap-6 md:gap-8") {
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
        footer(class="p-2 bg-slate-900 text-xs font-mono") {
            div(class="flex flex-row justify-between") {
                div { "© 2026 Luke Chu" }
                div(class="text-[9pt]") {
                    "Made with "
                    a(class="hover:underline font-bold text-red-200", href="https://rust-lang.org") { "Rust" }
                    " and "
                    a(class="hover:underline font-bold text-red-200", href="https://sycamore.dev") { "Sycamore" }
                }
            }
        }
    }
}

#[component]
pub fn NotFound() -> View {
    view! {
        div(class="max-w-prose mx-auto") {
            h1 { "404 Not Found" }
            p { "The page you are looking for does not exist." }
        }
    }
}
