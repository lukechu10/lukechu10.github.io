use sycamore::prelude::*;

#[component]
pub fn About() -> View {
    view! {
        h1(class="text-2xl font-bold") { "About" }
        p { "About me..." }
    }
}
