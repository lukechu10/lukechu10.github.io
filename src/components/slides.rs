//! Components for slides-format posts.

use mdsycx::FromMd;
use sycamore::prelude::*;

/// Context state used to manage slides.
#[derive(Debug, Clone, Copy)]
struct SlideState {
    current: Signal<usize>,
    total: Signal<usize>,
}

#[derive(Props, FromMd)]
pub struct SlideProps {
    pub children: Children,
}

#[component]
pub fn Slide(props: SlideProps) -> View {
    let children = props.children.call();
    view! {
        div(class="slide") {
            (children)
        }
    }
}

#[derive(Props, FromMd)]
pub struct SlideSegmentProps {
    pub children: Children,
}

#[component]
pub fn SlideSegment(props: SlideSegmentProps) -> View {
    let children = props.children.call();
    view! {
        div(class="slide-segment mb-4") {
            (children)
        }
    }
}
