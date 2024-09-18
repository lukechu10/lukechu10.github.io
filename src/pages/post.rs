use std::{collections::HashMap, sync::LazyLock};

use include_dir::{include_dir, Dir};
use mdsycx::{ComponentMap, ParseRes};
use serde::Deserialize;
use sycamore::prelude::*;
use sycamore_hooks::window::use_title;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PostDate {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

impl PartialOrd for PostDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.year, self.month, self.day).cmp(&(other.year, other.month, other.day)))
    }
}
impl Ord for PostDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.year, self.month, self.day).cmp(&(other.year, other.month, other.day))
    }
}

/// Deserialize date in format "YYYY-MM-DD"
fn deserialize_date<'de, D>(deserializer: D) -> Result<PostDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let mut parts = s.split('-').map(|x| x.parse().unwrap());
    Ok(PostDate {
        year: parts.next().unwrap(),
        month: parts.next().unwrap(),
        day: parts.next().unwrap(),
    })
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PostLayout {
    #[default]
    Prose,
    Full,
}

fn _render_math_default() -> bool {
    false
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct PostMetadata {
    pub title: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: PostDate,
    pub desc: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub layout: PostLayout,
    #[serde(default = "_render_math_default")]
    pub render_math: bool,
    /// The filename of the original markdown file. This is not deserialized by serde but populated
    /// manually.
    #[serde(skip)]
    pub filename: String,
}

static FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/posts");

pub static POSTS: LazyLock<HashMap<String, ParseRes<PostMetadata>>> = LazyLock::new(|| {
    FILES
        .find("**/*.mdx")
        .unwrap()
        .map(|dir| {
            let file = dir.as_file().unwrap();
            let contents = file.contents_utf8().expect("file not utf8");
            let mut parse_res: ParseRes<PostMetadata> =
                mdsycx::parse(contents).expect("parse failed");

            let filename = file
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            parse_res.front_matter.filename = filename.clone();
            (filename, parse_res)
        })
        .collect()
});

#[component(inline_props)]
pub fn PostView(id: String) -> View {
    let Some(post) = POSTS.get(&id) else {
        return view! {
            crate::shell::NotFound()
        };
    };
    // Code highlighting.
    on_mount(highlightAll);
    use_title(&format!("{} - lukechu", post.front_matter.title));

    // TODO: Only import MathJax if needed.
    if post.front_matter.render_math {
        on_mount(typeset);
    }

    let components = ComponentMap::new()
        .with("SlideShow", crate::components::slides::SlideShow)
        .with("Slide", crate::components::slides::Slide)
        .with("SlideSegment", crate::components::slides::SlideSegment)
        .with("span", crate::components::math::MathDisplay);

    match post.front_matter.layout {
        PostLayout::Prose => view! {
            div(class="post-content max-w-prose mx-auto") {
                crate::components::ShowDate(date=post.front_matter.date)

                mdsycx::MDSycX(body=post.body.clone(), components=components)
            }
        },
        PostLayout::Full => view! {
            div(class="post-content") {
                div(class="max-w-prose mx-auto") {
                    crate::components::ShowDate(date=post.front_matter.date)
                }

                mdsycx::MDSycX(body=post.body.clone(), components=components)
            }
        },
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightAll();

    #[wasm_bindgen(js_namespace = MathJax)]
    fn typeset();
}
