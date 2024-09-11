use sycamore::prelude::*;

use crate::pages::post::POSTS;

#[component]
pub fn Home() -> View {
    view! {
        h1(class="text-2xl font-bold") { "Home" }
        p { "Welcome to my website!" }

        h2(class="text-xl") { "Latest posts" }
        PostList()
    }
}

#[component]
pub fn PostList() -> View {
    view! {
        Indexed(
            list=POSTS.values().map(|x| x.front_matter.clone()).collect::<Vec<_>>(),
            view=|post| {
                view! {
                    div {
                        div {
                            h3(class="text-lg text-red-900") {
                                a(href=format!("/post/{}", post.id)) {
                                    (post.title.clone())
                                }
                            }
                            p(class="text-xs text-gray-500 -mt-1") { (post.date.clone()) }
                        }
                        p(class="text-sm") { (post.desc.clone()) }
                    }
                }
            }
        )
    }
}
