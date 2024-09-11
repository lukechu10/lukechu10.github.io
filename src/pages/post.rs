use std::{collections::HashMap, sync::LazyLock};

use include_dir::{include_dir, Dir};
use mdsycx::ParseRes;
use serde::Deserialize;
use sycamore::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct PostMetadata {
    pub title: String,
    pub id: String,
    pub date: String,
    pub desc: String,
}

static FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/posts");

pub static POSTS: LazyLock<HashMap<String, ParseRes<PostMetadata>>> = LazyLock::new(|| {
    FILES
        .files()
        .map(|file| {
            let contents = file.contents_utf8().expect("file not utf8");
            let parse_res: ParseRes<PostMetadata> = mdsycx::parse(contents).expect("parse failed");
            (parse_res.front_matter.id.clone(), parse_res)
        })
        .collect()
});

#[component(inline_props)]
pub fn PostView(id: String) -> View {
    let Some(post) = POSTS.get(&id) else {
        return view! {
            crate::NotFound()
        };
    };
    view! {
        div(class="post-content") {
            mdsycx::MDSycX(body=post.body.clone())
        }
    }
}
