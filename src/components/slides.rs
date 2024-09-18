//! Components for slides-format posts.

use std::str::FromStr;

use mdsycx::FromMd;
use sycamore::prelude::*;

#[derive(Props, FromMd)]
pub struct SlideShowProps {
    pub children: Children,
}

#[component]
pub fn SlideShow(props: SlideShowProps) -> View {
    let children = props.children.call();
    view! {
        div(class="slide-show") {
            (children)
        }
    }
}

/// Context state used to manage slides.
#[derive(Debug, Default, Clone, Copy)]
struct SlideState {
    current: Signal<usize>,
    total: Signal<usize>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum SlideKind {
    #[default]
    Text,
    Split,
}

impl FromStr for SlideKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Self::Text),
            "split" => Ok(Self::Split),
            _ => Err(()),
        }
    }
}

#[derive(Props, FromMd)]
pub struct SlideProps {
    pub kind: SlideKind,
    pub children: Children,
}

#[component]
pub fn Slide(props: SlideProps) -> View {
    let mut children = View::new();
    create_child_scope(|| {
        provide_context(SlideState::default());
        children = props.children.call();
    });

    match props.kind {
        SlideKind::Text => view! {
            div(class="slide max-w-prose mx-auto") {
                (children)
            }
        },
        SlideKind::Split => view! {
            div(class="slide grid grid-cols-2 gap-4 w-full") {
                div {
                    (children)
                }
                SlideGraphics()
            }
        },
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

#[component]
pub fn SlideGraphics() -> View {
    view! {
        div(class="slide-graphics aspect-video bg-slate-800 rounded-lg sticky mt-10 top-10") {
            "Video goes here"
        }
    }
}
