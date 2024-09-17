use sycamore::prelude::*;
use sycamore_hooks::window::use_title;

#[component]
pub fn About() -> View {
    use_title("About - lukechu");
    view! {
        div(class="max-w-prose mx-auto") {
            h1 { "About" }
            p { "About me..." }
        }
    }
}
