//! Components for slides-format posts.

use std::str::FromStr;

use mdsycx::FromMd;
use serde::Deserialize;
use sycamore::prelude::*;
use sycamore::web::Suspense;
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
        // Try to restore the slide number from the URL hash.
        let hash = window().location().hash().unwrap();
        let hash = hash.trim_start_matches("#slide-");
        let slide = hash.parse::<usize>().unwrap_or(0);

        let state = SlideShowState::default();
        state.current_slide.set(slide);

        // Create an effect that stores the slide number in the URL hash. We use this to restore the state when reloading the page.
        //
        // We put this in an on_mount to ensure that the effect runs after sycamore-router is done
        // because otherwise our URL will be overwritten.
        on_mount(move || {
            create_effect(move || {
                let current_slide = state.current_slide.get();
                window()
                    .history()
                    .unwrap()
                    .replace_state_with_url(
                        &JsValue::null(),
                        "",
                        Some(&format!("#slide-{current_slide}")),
                    )
                    .unwrap();
            });
        });

        provide_context(state);
        let children = props.children.call();

        view = view! {
            div(class="slide") {
                (children)
            }
            div(class="fixed bottom-0 left-0 bg-slate-950 w-full p-2 border-slate-700 border-t-2") {
                SlideControls()
            }
        };
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
    pub video_json: String,
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
                div(class="max-w-prose mx-auto md:mr-0") {
                    (children)
                }
                div(class="sticky mt-5 top-5 h-fit mx-auto md:ml-0") {
                    (if show() {
                        let video_json = props.video_json.clone();
                        let split = video_json.split('/').collect::<Vec<_>>();
                        if split.len() < 2 {
                            return view! { "Error loading video" };
                        }
                        let url_base = split[..split.len() - 2].join("/");
                        view! {
                            Suspense(fallback=|| "Loading...".into()) {
                                ManimSlide(url_base=url_base, json_src=video_json)
                            }
                        }
                    } else {
                        view! {}
                    })
                }
            }
        },
    };

    let class = "fixed top-0 left-0 px-3 pt-20 pb-10 h-full w-full overflow-y-auto overscroll-contain transition-opacity";
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
    // Register the slide segment.
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
    let class = "inline-block transition-opacity";
    let class = move || {
        if show() {
            class.to_string()
        } else {
            format!("{class} opacity-0 invisible h-0")
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

#[component(inline_props)]
pub fn Video<F: Fn() -> bool + 'static>(
    video: String,
    show: F,
    r#loop: bool,
    #[prop(attributes(html, video))] attributes: Attributes,
) -> View {
    let state = use_context::<SlideShowState>();

    let video_ref = create_node_ref();

    let show = create_selector(show);

    let class = move || {
        if show.get() {
            "aspect-video"
        } else {
            "aspect-video hidden"
        }
    };

    on_mount(move || {
        create_effect(move || {
            if show.get() {
                let video = video_ref
                    .get()
                    .unchecked_into::<web_sys::HtmlVideoElement>();
                let _ = video.play().unwrap();
            }
        });
    });

    let show_replay_btn = create_signal(false);
    let replay = move |_| {
        let video = video_ref.get().unchecked_into::<web_sys::HtmlVideoElement>();
        video.pause().unwrap();
        video.set_current_time(0.0);
        let _ = video.play().unwrap();
        show_replay_btn.set(false);
    };

    view! {
        div(class=class) {
            video(r#ref=video_ref, r#loop=r#loop, ..attributes, on:ended=move |_| if !r#loop { show_replay_btn.set(true); }) {
                source(src=video, r#type="video/mp4")
            }
            button(
                class=format!("bg-slate-800 font-mono text-sm rounded px-2 py-1 block mx-auto my-4 transition-opacity {}",
                    if show_replay_btn.get() && state.current_segment.get() != 0 { "opacity-100" } else { "invisible opacity-0" }
                ),
                on:click=replay,
            ) {
                "Replay"
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct ManimSlides {
    slides: Vec<ManimSlideData>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
struct ManimSlideData {
    file: String,
    #[serde(rename = "loop")]
    r#loop: bool,
    auto_next: bool,
}

#[component(inline_props)]
pub async fn ManimSlide(url_base: String, json_src: String) -> View {
    let mut state = use_context::<SlideShowState>();

    // Fetch the file over HTTP and parse it.
    let response = gloo_net::http::Request::get(&json_src)
        .send()
        .await
        .expect("could not fetch slide"); // TODO: handle error
    let slides: ManimSlides = response.json().await.expect("could not parse slide");

    view! {
        div {
            Indexed(
                list=slides.slides.into_iter().enumerate().collect::<Vec<_>>(),
                view=move |(i, slide)| {
                    let src = format!("{url_base}/{}", slide.file);
                    view! {
                        Video(
                            video=src,
                            r#loop=slide.r#loop,
                            show=move || state.current_segment.get() == i,
                            on:ended=move |_| {
                                if slide.auto_next {
                                    state.current_segment += 1;
                                }
                            },
                        )
                    }
                }
            )
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
        div(class="m-auto text-xs font-mono flex flex-row") {
            div(class="flex-grow flex flex-row justify-center gap-10") {
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
            span(class="flex-initial") {
                (state.current_slide.get() + 1) " / " (state.slides.with(Vec::len))
            }
        }
    }
}
