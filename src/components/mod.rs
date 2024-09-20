pub mod math;
pub mod slides;

use mdsycx::FromMd;
use sycamore::prelude::*;

use crate::pages::post::PostDate;

#[derive(Props, FromMd)]
pub struct ShowDateProps {
    pub date: PostDate,
}

/// Display date string
#[component]
pub fn ShowDate(ShowDateProps { date }: ShowDateProps) -> View {
    let day = date.day.to_string();

    static MONTHS: &[&str] = &[
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let month = MONTHS.get(date.month as usize - 1).expect("invalid month");

    let year = date.year.to_string();

    view! {
        p(class="text-sm text-gray-400 !mb-0 font-mono") { (format!("{month} {day}, {year}")) }
    }
}
