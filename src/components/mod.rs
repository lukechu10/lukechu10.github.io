pub mod slides;

use sycamore::prelude::*;

use crate::pages::post::PostDate;

/// Display date string
#[component(inline_props)]
pub fn ShowDate(date: PostDate) -> View {
    let day = date.day.to_string();

    static MONTHS: &[&str] = &[
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let month = MONTHS.get(date.month as usize - 1).expect("invalid month");

    let year = date.year.to_string();


    view! {
        p(class="text-sm text-gray-400 -mt-1 font-mono") { (format!("{month} {day}, {year}")) }
    }
}

