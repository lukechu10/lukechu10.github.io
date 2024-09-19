//! Components for slides-format posts.

use std::str::FromStr;

use mdsycx::FromMd;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

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
    let mut children = View::new();
    create_child_scope(|| {
        provide_context(SlideShowState::default());
        children = props.children.call();
    });

    on_mount(|| {
        let body = document().body().unwrap();
        body.class_list().add_1("overflow-hidden").unwrap();
        on_cleanup(move || {
            body.class_list().remove_1("overflow-hidden").unwrap();
        });
    });

    view! {
        div(class="mb-[100vh]") {
            (children)
        }
    }
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
            div(class="max-w-prose mx-auto min-h-screen") {
                (children)
                SlideControls()
            }
        },
        SlideKind::Split => view! {
            div(class="grid grid-cols-2 gap-4 w-full min-h-screen content-center") {
                div(class="max-w-prose ml-auto") {
                    (children)
                }
                div(class="sticky mt-5 top-5 max-w-prose h-fit") {
                    SlideGraphics()
                    SlideControls()
                }
            }
        },
    };
    view! {
        div(class="slide mb-10") {
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

    view! {
        div(class="slide-segment mb-4", r#ref=start) {
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
            "inline-block hover:underline transition-transform ease-in-out delay-50 hover:-translate-x-0.5"
        } else {
            "hidden"
        }
    };
    let next_class = move || {
        if has_next() {
            "inline-block hover:underline transition-transform ease-in-out delay-50 hover:translate-x-0.5"
        } else {
            "hidden"
        }
    };

    let scroll_to_current = move || {
        if state.current_slide.get() == 0 && state.current_segment.get() == 0 {
            let scroll_options = web_sys::ScrollToOptions::new();
            scroll_options.set_top(0.0);
            scroll_options.set_behavior(web_sys::ScrollBehavior::Smooth);
            window().scroll_to_with_scroll_to_options(&scroll_options);
        } else {
            let scroll_options = web_sys::ScrollIntoViewOptions::new();
            scroll_options.set_behavior(web_sys::ScrollBehavior::Smooth);
            let current_segment = state.slides.with(|slides| {
                slides[state.current_slide.get()].starts[state.current_segment.get()]
            });
            current_segment
                .get()
                .unchecked_into::<web_sys::Element>()
                .scroll_into_view_with_scroll_into_view_options(&scroll_options);
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

        scroll_to_current();
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

        scroll_to_current();
    };

    view! {
        div(class="block mx-auto text-sm flex flex-row justify-center gap-10") {
            button(
                class=previous_class,
                on:click=previous,
            ) {
                "< Previous"
            }
            button(
                class=next_class,
                on:click=next,
            ) {
                "Next >"
            }
        }
    }
}
