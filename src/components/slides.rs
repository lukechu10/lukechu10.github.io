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
    pub segments: Vec<SlideSegmentData>,
}

#[derive(Debug, Clone)]
struct SlideSegmentData {}

#[derive(Props, FromMd)]
pub struct SlideShowProps {
    pub children: Children,
}

#[component]
pub fn SlideShow(props: SlideShowProps) -> View {
    let mut view = View::default();

    // Prevent overflow on the body.
    let body = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();
    body.class_list().add_1("overflow-hidden").unwrap();
    on_cleanup(move || {
        body.class_list().remove_1("overflow-hidden").unwrap();
    });

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
    pub video: String,
    pub children: Children,
}

#[component]
pub fn Slide(props: SlideProps) -> View {
    // Register the slide.
    let state = use_context::<SlideShowState>();
    let slide_number = state.slides.update(|slides| {
        slides.push(SlideData {
            segments: Vec::new(),
        });
        slides.len() - 1
    });

    let show = move || state.current_slide.get() == slide_number;

    let children = props.children.call();

    let slide_content = match props.kind {
        SlideKind::Text => view! {
            div(class="max-w-prose mx-auto") {
                (children)
            }
        },
        SlideKind::Split => view! {
            div(class="grid grid-flow-row md:grid-flow-col md:grid-cols-2 md:content-center gap-4 w-full ") {
                div(class="max-w-prose mx-auto md:mr-0 overflow-y-auto") {
                    (children)
                }
                div(class="sticky mt-5 top-5 h-fit mx-auto md:ml-0") {
                    (if show() {
                        let video = props.video.clone();
                        view! {
                            SlideGraphics(video=video)
                        }
                    } else {
                        view! {}
                    })
                }
            }
        },
    };

    let class = "fixed top-0 left-0 px-3 py-20 h-full w-full overflow-y-auto overscroll-contain transition-opacity";
    let class = move || {
        if show() {
            class.to_string()
        } else {
            format!("{class} opacity-0 invisible")
        }
    };
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
    // Register ths slide segment.
    let state = use_context::<SlideShowState>();
    let (slide_number, segment_number) = state.slides.update(|slides| {
        let slide_number = slides.len() - 1;
        let current_slide = slides
            .last_mut()
            .expect("SlideSegment must be nested under a Slide");
        current_slide.segments.push(SlideSegmentData {});
        (slide_number, current_slide.segments.len() - 1)
    });

    let show = move || {
        state.current_slide.get() == slide_number && state.current_segment.get() >= segment_number
    };
    let class = "transition-opacity";
    let class = move || {
        if show() {
            class.to_string()
        } else {
            format!("{class} opacity-0 invisible")
        }
    };

    let children = props.children.call();

    view! {
        span(class=class) {
            (children)
        }
    }
}

#[derive(Props, FromMd)]
pub struct NextSegmentLinkProps {
    pub children: Children,
}

#[component]
pub fn NextSegmentLink(props: NextSegmentLinkProps) -> View {
    let mut state = use_context::<SlideShowState>();
    let (current_slide, current_segment) = state
        .slides
        .with(|slides| (slides.len() - 1, slides.last().unwrap().segments.len() - 1));

    let active = move || {
        state.current_slide.get() == current_slide && state.current_segment.get() == current_segment
    };

    let on_click = move |_| {
        if active() {
            state.current_segment += 1;
        }
    };

    let class = move || {
        if active() {
            "hover:underline text-red-300 cursor-pointer"
        } else {
            "text-gray-400"
        }
    };

    let children = props.children.call();
    view! {
        span(class=class, on:click=on_click) {
            (children)
        }
    }
}

#[derive(Props, FromMd)]
pub struct SlideGraphicsProps {
    pub video: String,
}

#[component]
pub fn SlideGraphics(props: SlideGraphicsProps) -> View {
    let video_ref = create_node_ref();
    view! {
        div(class="aspect-video") {
            (if !props.video.is_empty() {
                let video = props.video.clone();
                view! {
                    video(autoplay=true, r#ref=video_ref) {
                        source(src=video, r#type="video/mp4")
                    }
                }
            } else {
                view! { "No video..." }
            })
        }
    }
}

#[component]
pub fn SlideControls() -> View {
    let mut state = use_context::<SlideShowState>();

    let has_previous = move || state.current_slide.get() > 0;
    let has_next = move || state.current_slide.get() < state.slides.with(Vec::len) - 1;

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
        state.current_slide -= 1;
        state.current_segment.set(0);
    };
    let next = move |_| {
        assert!(has_next());
        state.current_slide += 1;
        state.current_segment.set(0);
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
