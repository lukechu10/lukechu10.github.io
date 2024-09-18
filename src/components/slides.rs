//! Components for slides-format posts.

use mdsycx::FromMd;
use sycamore::prelude::*;

/// Context state used to manage slides.
#[derive(Debug, Default, Clone, Copy)]
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
    provide_context(SlideState::default());

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
    let mut state = use_context::<SlideState>();
    // Register the slide segment.
    state.total += 1;
    
    let children = props.children.call();
    view! {
        div(class="slide-segment mb-4") {
            (children)
        }
    }
}
