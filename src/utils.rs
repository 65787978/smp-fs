use dioxus::prelude::*;
use gloo::{timers::future::TimeoutFuture, utils::document};

use crate::data::structs::MinerStats;

#[derive(PartialEq, Eq, Clone)]
pub struct ParagraphCard {
    pub classes: String,
    pub text: String,
}

#[derive(PartialEq, Eq, Clone)]
pub struct InfoCard {
    pub classes: String,
    pub value: String,
    pub unit: String,
    pub heading: String,
}

#[derive(PartialEq, Eq, Clone)]
pub struct InfoCardDouble {
    pub classes: String,
    pub value_1: String,
    pub unit_1: String,
    pub heading_1: String,
    pub value_2: String,
    pub unit_2: String,
    pub heading_2: String,
    pub explanation_bubble: bool,
    pub bubble_text: String,
}

#[derive(PartialEq, Eq, Clone)]
pub struct InfoCardPlaceholder {
    pub heading: String,
}
#[component]
pub fn InfoCardDouble(vars: InfoCardDouble) -> Element {
    let InfoCardDouble {
        classes,
        value_1,
        unit_1,
        heading_1,
        value_2,
        unit_2,
        heading_2,
        explanation_bubble,
        bubble_text,
    } = vars;

    rsx! {
        div {class:"min-h-[6rem] text-center text-slate-200 rounded-lg {classes} bg-opacity-25 bg-black backdrop-filter backdrop-blur-md shadow-lg m-2",
                div {class:"flex justify-around m-6",

                    div {
                        div {class:"text-2xl", "{value_1}", a {class:"text-sm", "{unit_1}"}}
                        a {class:"text-m", "{heading_1}"}
                    }
                    div {
                        div {class:"text-2xl", "{value_2}", a {class:"text-sm", "{unit_2}"}}
                        a {class:"text-m", "{heading_2}"}
                    }

                },

                {if explanation_bubble { ExplanationBubble(bubble_text)} else {rsx!()}}
        }
    }
}

#[component]
pub fn InfoCard(vars: InfoCard) -> Element {
    let InfoCard {
        classes,
        value,
        unit,
        heading,
    } = vars;

    rsx! {
        div {class:"min-h-[4rem] text-center text-slate-200 rounded-lg {classes} bg-opacity-25 bg-black backdrop-filter backdrop-blur-md shadow-lg m-2 py-0.5",
            div {class:"flex justify-around m-6",
                div {
                    div {class:"text-2xl", "{value}", a {class:"text-sm", "{unit}"}}
                    a {class:"text-m", "{heading}"}
                }
            }
        }
    }
}

#[component]
pub fn ParagraphCard(vars: ParagraphCard) -> Element {
    let ParagraphCard { classes, text } = vars;
    rsx! {
        div {class:"min-h-[8rem] max-w-2xl rounded-lg {classes} ",
            p {"{text}"}
        },
    }
}

#[component]
pub fn InfoCardPlaceholder(vars: InfoCardPlaceholder) -> Element {
    let InfoCardPlaceholder { heading } = vars;

    rsx! {
        div {class:"min-h-[8rem] text-center text-slate-200 rounded-lg bg-opacity-25 bg-black backdrop-filter backdrop-blur-md shadow-lg m-2",
            div {class:"flex justify-around m-6",
                div {class:"flex items-center justify-center",
                    div {class:"animate-spin rounded-full h-10 w-10 border-t-2 border-b-2 border-slate-300",i {class:"h-5 w-5 text-slate-300 mt-0.5"}}
                }
                a {class:"text-m", "{heading}"}
            }
        }
    }
}

fn ExplanationBubble(text: String) -> Element {
    let mut bubble_state = use_signal(|| false);
    let mut explanation_bubble_style = use_signal(|| {
        "visibility: hidden; opacity: 0; transition: visibility 0s, opacity 0.2s linear"
    });

    rsx! {
            div {
                div {
                    onclick: move |_| {
    /*
                            if bubble_state() {
                                let explanation_bubble = document().get_element_by_id("explanation-bubble");
                                if let Some(explanation_bubble) = explanation_bubble {
                                explanation_bubble.set_attribute("style", "visibility: hidden; opacity: 0");
                                }
                                bubble_state.set(false);
                            }
                            else {
                                let explanation_bubble = document().get_element_by_id("explanation-bubble");
                                if let Some(explanation_bubble) = explanation_bubble {
                                explanation_bubble.set_attribute("style", "visibility: visible; opacity: 1");
                                }
                                bubble_state.set(true);
                            }
    */

                        if bubble_state() {
                            explanation_bubble_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear");
                            bubble_state.set(false);
                        }
                        else {
                            explanation_bubble_style.set("visibility: visible; opacity: 1; transition: visibility 1s, opacity 0.2s linear");
                            bubble_state.set(true);
                        }

                    },
                    class: "fixed top-0 right-0 bottom-0 overflow-y-auto p-2",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg", width: "16", height: "16", fill: "currentColor", class: "bi bi-question-circle-fill", "viewBox": "0 0 16 16",
                        path {d: "M8 16A8 8 0 1 0 8 0a8 8 0 1 0 0 16zm.93-9.412-1 4.705c-.07.34.029.533.304.533.194 0 .487-.07.68-.246l-.088.416c-.287.346-.92.598-1.465.598-.707 0-1.002-.422-.808-1.319l.738-3.468c.064-.293.006-.399-.287-.47l-.451-.081.082-.381 2.29-.287z"}
                    },

                    div {
                        class: "fixed top-0 left-0 right-0 bottom-0 overflow-y-auto",
                        div {
                            class: "text-sm text-center text-slate-50 rounded-lg bg-opacity-30 bg-gray backdrop-filter backdrop-blur-md shadow-lg p-2",
                            style: "{explanation_bubble_style}",
                            id: "explanation-bubble",
                            "{text}"
                        },
                    }
                }


            }
        }
}
