use sycamore::prelude::*;

#[component]
pub fn About() -> View {
    view! {
        div(class="max-w-prose mx-auto") {
            h1 { "About" }
            p { "About me..." }
        }
    }
}
