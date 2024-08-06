use dioxus::prelude::*;
use gloo::utils::document;

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
        div {class:"min-h-[8rem] text-center text-slate-200 rounded-lg {classes} bg-opacity-15 bg-white backdrop-filter backdrop-blur-md shadow-lg m-2",
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
        div {class:"min-h-[8rem] text-center text-slate-200 rounded-lg {classes} bg-opacity-15 bg-white backdrop-filter backdrop-blur-md shadow-lg m-2",
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

fn ExplanationBubble(text: String) -> Element {
    let mut bubble_state = use_signal(|| false);

    rsx! {
        div {
            div {
                onclick: move |_| {

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
                        style: "visibility: hidden; opacity: 0; transition: visibility 0s, opacity 0.2s linear",
                        id: "explanation-bubble",
                        "{text}"
                    },
                }
            }


        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {class:"min-h-[2rem] inset-x-0 bottom-0 bg-opacity-10 bg-white backdrop-filter backdrop-blur-md fixed",
            div {class:"flex flex-row justify-center",
                    a {class:"icon m-2", href:"https://github.com/th3-cr34t0r/smp-fs",
                    svg {
                        xmlns:"http://www.w3.org/2000/svg", width:"25", height:"25", fill:"white", class:"bi bi-github", "viewBox":"0 0 16 16",
                        path {d:"M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8"}
                    }
                }

                a {class:"icon m-2", href:"https://discord.com/channels/668903786361651200/1153460448214122526",
                    svg {
                        xmlns:"http://www.w3.org/2000/svg", width:"25", height:"25", fill:"white", class:"bi bi-discord", "viewBox":"0 0 16 16",
                        path {d:"M13.545 2.907a13.2 13.2 0 0 0-3.257-1.011.05.05 0 0 0-.052.025c-.141.25-.297.577-.406.833a12.2 12.2 0 0 0-3.658 0 8 8 0 0 0-.412-.833.05.05 0 0 0-.052-.025c-1.125.194-2.22.534-3.257 1.011a.04.04 0 0 0-.021.018C.356 6.024-.213 9.047.066 12.032q.003.022.021.037a13.3 13.3 0 0 0 3.995 2.02.05.05 0 0 0 .056-.019q.463-.63.818-1.329a.05.05 0 0 0-.01-.059l-.018-.011a9 9 0 0 1-1.248-.595.05.05 0 0 1-.02-.066l.015-.019q.127-.095.248-.195a.05.05 0 0 1 .051-.007c2.619 1.196 5.454 1.196 8.041 0a.05.05 0 0 1 .053.007q.121.1.248.195a.05.05 0 0 1-.004.085 8 8 0 0 1-1.249.594.05.05 0 0 0-.03.03.05.05 0 0 0 .003.041c.24.465.515.909.817 1.329a.05.05 0 0 0 .056.019 13.2 13.2 0 0 0 4.001-2.02.05.05 0 0 0 .021-.037c.334-3.451-.559-6.449-2.366-9.106a.03.03 0 0 0-.02-.019m-8.198 7.307c-.789 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.45.73 1.438 1.613 0 .888-.637 1.612-1.438 1.612m5.316 0c-.788 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.451.73 1.438 1.613 0 .888-.631 1.612-1.438 1.612"}
                    }
                }

                a {class:"icon m-2", href:"https://t.me/sig_mining",
                    svg {
                        xmlns:"http://www.w3.org/2000/svg", width:"25", height:"25", fill:"white", class:"bi bi-telegram", "viewBox":"0 0 16 16",
                        path {d:"M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0M8.287 5.906q-1.168.486-4.666 2.01-.567.225-.595.442c-.03.243.275.339.69.47l.175.055c.408.133.958.288 1.243.294q.39.01.868-.32 3.269-2.206 3.374-2.23c.05-.012.12-.026.166.016s.042.12.037.141c-.03.129-1.227 1.241-1.846 1.817-.193.18-.33.307-.358.336a8 8 0 0 1-.188.186c-.38.366-.664.64.015 1.088.327.216.589.393.85.571.284.194.568.387.936.629q.14.092.27.187c.331.236.63.448.997.414.214-.02.435-.22.547-.82.265-1.417.786-4.486.906-5.751a1.4 1.4 0 0 0-.013-.315.34.34 0 0 0-.114-.217.53.53 0 0 0-.31-.093c-.3.005-.763.166-2.984 1.09"}
                    }
                }
            }
        }
    }
}

#[component]
pub fn Chart(chart_data: Vec<(String, String)>) -> Element {
    let mut x_axis = use_signal(|| vec![String::new()]);
    let mut y_axis = use_signal(|| vec![String::new()]);

    for data in chart_data {
        x_axis.push(data.0);
        y_axis.push(data.1);
    }

    let future = use_resource(move || async move {
        let mut chart = eval(
            r#"

                    let x_axis_data = await dioxus.recv();
                    let y_axis_data = await dioxus.recv();


                    var ctx = document.getElementById('myChart');

                    new Chart(ctx, {
                        type: 'line',
                        data: {
                            labels: x_axis_data,
                            datasets: [{
                                label: 'Miner Hashrate',
                                data:  y_axis_data,
                                borderColor: 'rgba(238, 238, 238, 0.93)',
                                tension: 0.5,
                                borderWidth: 2,
                                pointStyle: false,
                                fill: true
                            }]
                        },
                        options: {
                            maintainAspectRatio: false,
                            scales: {
                                y: {
                                    beginAtZero: true,
                                    max: Math.round(y_axis_data[0] / 1000) * 1500
                                }
                            }
                        }
                    });
                "#,
        );

        // Send a message to the JS code.
        chart.send(x_axis().into()).unwrap();
        chart.send(y_axis().into()).unwrap();

        // Our line on the JS side will log the message and then return the chart.
        let res = chart.recv().await.unwrap();

        res
    });

    rsx! {

        div { class:"max-h-20rem max-w-xl text-center text-slate-200 rounded-lg bg-opacity-15 bg-white backdrop-filter backdrop-blur-md shadow-lg m-2",
            div {class:"flex m-2",
                canvas {id: "myChart"}

                match future.value().as_ref() {
                    Some(chart) => rsx!{"{chart}"},
                    _ => rsx!{"Loading..."}
                }
            }

        }
    }
}
