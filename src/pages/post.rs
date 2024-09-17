use std::{collections::HashMap, sync::LazyLock};

use include_dir::{include_dir, Dir};
use mdsycx::ParseRes;
use wasm_bindgen::prelude::*;
use serde::Deserialize;
use sycamore::prelude::*;

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
fn deerialize_date<'de, D>(deserializer: D) -> Result<PostDate, D::Error>
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct PostMetadata {
    pub title: String,
    #[serde(deserialize_with = "deerialize_date")]
    pub date: PostDate,
    pub desc: String,
    pub tags: Vec<String>,
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
            let mut parse_res: ParseRes<PostMetadata> = mdsycx::parse(contents).expect("parse failed");

            let filename = file.path().file_stem().unwrap().to_str().unwrap().to_string();
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
    on_mount(move || {
        highlightAll();
    });
    view! {
        div(class="max-w-prose mx-auto") {
            crate::components::ShowDate(date=post.front_matter.date)
            div(class="post-content") {
                mdsycx::MDSycX(body=post.body.clone())
            }
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightAll();
}
