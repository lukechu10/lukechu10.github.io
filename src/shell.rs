use sycamore::prelude::*;

#[component]
pub fn Header() -> View {
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

#[component]
pub fn Footer() -> View {
    view! {
        footer(class="p-2 border-t-2 border-gray-200 text-sm") {
            div(class="flex flex-row justify-between") {
                div { "© 2024 Luke Chu" }
                div {
                    "Made with "
                    a(href="https://rust-lang.org") { "Rust" }
                    " and "
                    a(href="https://github.com/sycamore-rs/sycamore") { "Sycamore" }
                }
            }
        }
    }
}
