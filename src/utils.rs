use dioxus::prelude::*;

pub fn InfoCard(min_h: &str, classes: &str, value: &str, unit: &str, heading: &str) -> Element {
    rsx! {
        div {class:"min-h-[{min_h}rem] rounded-lg {classes} bg-opacity-30 bg-white backdrop-filter backdrop-blur-md shadow-lg",
            div {class:"grid grid-col-1 m-10",
                a {class:"text-2xl", "{value}", a {class:"text-sm", "{unit}"}}
                p {class:"text-m", "{heading}"}
            }
        },
    }
}

pub fn Card(min_h: &str, classes: &str, text: String) -> Element {
    rsx! {
        div {class:"min-h-[{min_h}rem] max-w-2xl rounded-lg {classes} ",
            p {"{text}"}
        },
    }
}

pub fn Footer() -> Element {
    rsx! {
    div {class:"min-h-[2rem] inset-x-0 bottom-0 bg-opacity-20 bg-white backdrop-filter backdrop-blur-md fixed", "Footer"}}
}
