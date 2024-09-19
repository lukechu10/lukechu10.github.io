//! Components for slides-format posts.

use std::str::FromStr;

use mdsycx::FromMd;
use sycamore::prelude::*;

/// Context state used to manage slides.
#[derive(Debug, Default, Clone, Copy)]
struct SlideShowState {
    slides: Signal<Vec<SlideData>>,
    current_slide: Signal<usize>,
    current_segment: Signal<usize>,
}

#[derive(Debug, Clone)]
struct SlideData {
    starts: Vec<NodeRef>,
    segments: usize,
}

#[derive(Props, FromMd)]
pub struct SlideShowProps {
    pub children: Children,
}

#[component]
pub fn SlideShow(props: SlideShowProps) -> View {
    let mut view = View::default();
    create_child_scope(|| {
        provide_context(SlideShowState::default());
        let children = props.children.call();

        view = view! {
            div(class="slide") {
                (children)
            }
            div(class="fixed bottom-0 left-0 bg-slate-950 w-full p-2 border-slate-700 border-t-2") {
                SlideControls()
            }
        }
    });
    view
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
    // Register the slide.
    let state = use_context::<SlideShowState>();
    state.slides.update(|slides| {
        slides.push(SlideData {
            starts: Vec::new(),
            segments: 0,
        })
    });

    let children = props.children.call();

    assert!(
        state
            .slides
            .with(|slides| slides.last().unwrap().segments > 0),
        "slide must have at least one segment"
    );

    let slide_content = match props.kind {
        SlideKind::Text => view! {
            div(class="max-w-prose mx-auto") {
                (children)
            }
        },
        SlideKind::Split => view! {
            div(class="grid grid-cols-2 gap-4 w-full content-center") {
                div(class="max-w-prose ml-auto overflow-y-auto") {
                    (children)
                }
                div(class="sticky mt-5 top-5 max-w-prose h-fit") {
                    SlideGraphics()
                }
            }
        },
    };

    let slide_number = state.slides.with(|slides| slides.len() - 1);
    let show = move || state.current_slide.get() == slide_number;
    let class = move || if show() { "" } else { "hidden" };
    view! {
        div(class=class) {
            (slide_content)
        }
    }
}

#[derive(Props, FromMd)]
pub struct SlideSegmentProps {
    pub children: Children,
}

#[component]
pub fn SlideSegment(props: SlideSegmentProps) -> View {
    let start = create_node_ref();

    // Register the slide segment.
    let state = use_context::<SlideShowState>();
    state.slides.update(|slides| {
        let slide = slides.last_mut().expect("SlideSegment must be in Slide");
        slide.segments += 1;
        slide.starts.push(start);
    });

    let children = props.children.call();

    let slide_number = state.slides.with(|slides| slides.len() - 1);
    let segment_number = state
        .slides
        .with(|slides| slides[slide_number].segments - 1);
    let show = move || {
        state.current_slide.get() == slide_number && state.current_segment.get() == segment_number
    };
    let class = move || if show() { "mb-4" } else { "mb-4 hidden" };

    view! {
        div(class=class, r#ref=start) {
            (children)
        }
    }
}

#[component]
pub fn SlideGraphics() -> View {
    view! {
        div(class="aspect-video") {
            p(class="p-5 font-sans") {
                "TODO: Video"
            }
        }
    }
}

#[component]
pub fn SlideControls() -> View {
    let mut state = use_context::<SlideShowState>();

    let has_previous = move || state.current_slide.get() > 0 || state.current_segment.get() > 0;
    let has_next = move || {
        state.current_segment.get()
            < state
                .slides
                .with(|slides| slides[state.current_slide.get()].segments)
                - 1
            || state.current_slide.get() < state.slides.with(Vec::len) - 1
    };

    let previous_class = move || {
        if has_previous() {
            "hover:underline transition-transform ease-in-out delay-50 hover:-translate-x-0.5"
        } else {
            "text-gray-400"
        }
    };
    let next_class = move || {
        if has_next() {
            "hover:underline transition-transform ease-in-out delay-50 hover:translate-x-0.5"
        } else {
            "text-gray-400"
        }
    };

    let previous = move |_| {
        assert!(has_previous());
        if state.current_segment.get() > 0 {
            state.current_segment -= 1;
        } else {
            state.current_slide -= 1;
            state.current_segment.set(0);
        }
    };
    let next = move |_| {
        assert!(has_next());
        // Get next segment and scroll to it.
        if state.current_segment.get()
            < state
                .slides
                .with(|slides| slides[state.current_slide.get()].segments)
                - 1
        {
            state.current_segment += 1;
        } else {
            state.current_slide += 1;
            state.current_segment.set(0);
        }
    };

    view! {
        div(class="block m-auto text-sm flex flex-row justify-center gap-10") {
            button(
                class=previous_class,
                disabled=!has_previous(),
                on:click=previous,
            ) {
                "< Previous"
            }
            button(
                class=next_class,
                disabled=!has_next(),
                on:click=next,
            ) {
                "Next >"
            }
        }
    }
}
