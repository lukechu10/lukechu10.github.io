use sycamore::prelude::*;

use crate::pages::post::POSTS;

#[component]
pub fn Home() -> View {
    view! {
        PostList()
    }
}

#[component]
fn PostList() -> View {
    let mut posts = POSTS.values().map(|x| x.front_matter.clone()).collect::<Vec<_>>();
    // Sort posts by date descending.
    posts.sort_by_key(|post| post.date);
    view! {
        ul(class="max-w-prose mx-auto") {
            Indexed(
                list=posts.into_iter().rev().collect::<Vec<_>>(),
                view=|post| {
                    view! {
                        li(class="mb-10") {
                            crate::components::ShowDate(date=post.date)
                            h1 {
                                a(class="hover:underline", href=format!("/post/{}", post.filename)) {
                                    (post.title.clone())
                                }
                            }
                            p { (post.desc.clone()) }
                            div(class="text-xs text-gray-400 font-mono") {
                                Indexed(
                                    list=post.tags,
                                    view=|tag| {
                                        view! {
                                            span { "#" (tag) }
                                        }
                                    }
                                )
                            }
                        }
                    }
                }
            )
        }
    }
}
