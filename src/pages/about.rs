use sycamore::prelude::*;
use sycamore_hooks::window::use_title;

#[component]
pub fn About() -> View {
    use_title("About - lukechu");
    view! {
        div(class="post-content max-w-prose mx-auto") {
            h1 { "About" }
            p {
                "Hi there!"
            }
            p {
                "I'm Luke — currently pursuing a master's degree in Mathematics and Theoretical Physics, and equally passionate about computers."
            }
            p {
                "I created and maintain "
                a(href="https://github.com/sycamore-rs/sycamore", target="_blank") { "Sycamore" }
                ", a Rust UI framework based on reactive functional programming. In fact, this very website is built using Sycamore!"
            }
            p {
                "For inquiries, you can reach me at "
                a(href="mailto:me@lukechu.dev") { "me@lukechu.dev" }
                "."
            }
        }
    }
}
