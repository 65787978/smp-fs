use dioxus::prelude::*;

pub fn InfoCardDouble(
    min_h: &str,
    classes: &str,
    value_1: &str,
    unit_1: &str,
    heading_1: &str,
    value_2: &str,
    unit_2: &str,
    heading_2: &str,
) -> Element {
    rsx! {
        div {class:"min-h-[{min_h}rem] rounded-lg {classes} bg-opacity-30 bg-white backdrop-filter backdrop-blur-md shadow-lg",
            div {class:"grid grid-col-2 m-1",
                div {
                    a {class:"text-2xl", "{value_1}", a {class:"text-sm", "{unit_1}"}}
                    p {class:"text-m", "{heading_1}"}
                }
                div {
                    a {class:"text-2xl", "{value_2}", a {class:"text-sm", "{unit_2}"}}
                    p {class:"text-m", "{heading_2}"}
                }
            }
        },
    }
}

pub fn InfoCard_u64(min_h: &str, classes: &str, value: u64, unit: &str, heading: &str) -> Element {
    rsx! {
        div {class:"min-h-[{min_h}rem] rounded-lg {classes} bg-opacity-30 bg-white backdrop-filter backdrop-blur-md shadow-lg",
            div {class:"grid grid-col-1 m-10",
                a {class:"text-2xl", "{value}", a {class:"text-sm", "{unit}"}}
                p {class:"text-m", "{heading}"}
            }
        },
    }
}

pub fn InfoCard_f64(min_h: &str, classes: &str, value: f64, unit: &str, heading: &str) -> Element {
    rsx! {
        div {class:"min-h-[{min_h}rem] rounded-lg {classes} bg-opacity-30 bg-white backdrop-filter backdrop-blur-md shadow-lg",
            div {class:"grid grid-col-1 m-10",
                a {class:"text-2xl", "{value}", a {class:"text-sm", "{unit}"}}
                p {class:"text-m", "{heading}"}
            }
        },
    }
}

pub fn InfoCard_str(min_h: &str, classes: &str, value: &str, unit: &str, heading: &str) -> Element {
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
