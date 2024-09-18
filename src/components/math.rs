use mdsycx::FromMd;
use sycamore::prelude::*;

#[derive(Props, FromMd)]
pub struct MathDisplayProps {
    pub children: Children,
    pub class: String,
}

/// Display math content. If `class` is "math math-display", the content is wrapped in `$$`.
///
/// This allows MathJax to detect and render the content as math.
#[component]
pub fn MathDisplay(props: MathDisplayProps) -> View {
    let children = props.children.call();
    let class = props.class;

    if class == "math math-display" {
        view! {
            span(class=class) {
                "$$" (children) "$$"
            }
        }
    } else {
        view! {
            span(class=class) {
                (children)
            }
        }
    }
}
