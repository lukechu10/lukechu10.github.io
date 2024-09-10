use sycamore::prelude::*;

#[component]
fn App() -> View {
    view! {
        "Luke's Blog"
    }
}

fn main() {
    sycamore::render(App);
}
