//! Run with:
//!
//! ```sh
//! dx serve --platform fullstack
//! ```

#![allow(non_snake_case, unused)]

mod data {
    pub mod api_data;
    pub mod structs;
}
mod routes {
    pub mod blocks;
    pub mod home;
    pub mod miner;
}
mod utils;

use data::api_data;
use routes::blocks::BlockPage;
use routes::home::HomePage_slice;
use routes::miner::MinerPage_slice;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use utils::*;

use dioxus::prelude::LaunchBuilder;
use dioxus::prelude::*;
use dioxus_fullstack::Config;
use serde::{Deserialize, Serialize};

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

/* Global Signals */
static WINDOW_DIMS: GlobalSignal<(f64, f64)> = Signal::global(|| (0.0, 0.0));
static NAV_INIT_STATE_FLAG: GlobalSignal<bool> = Signal::global(|| false);

#[derive(Clone, Routable, Debug, PartialEq, Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        HomePage {},
        #[route("/blocks")]
        BlockPage {},
        #[route("/wallet/:address")]
        MinerPage { address: String },
    #[end_layout]
    #[route("/:route")]
    PageNotFound {
        route: String,
    },
}

fn app() -> Element {
    // Start screen size watcher
    use_future(move || async move {
        let mut eval = eval(
            r"
                function resize() {
                    dioxus.send([window.innerWidth, window.innerHeight]);
                }
                window.addEventListener('resize', resize);
                dioxus.send([window.innerWidth, window.innerHeight]);
            ",
        );
        loop {
            let response = eval.recv().await.unwrap();
            let dims: (f64, f64) = serde_json::from_value(response).unwrap();
            *WINDOW_DIMS.write() = (dims.0, dims.1);
        }
    });

    rsx!(
        div { class: "bg-cover bg-no-repeat bg-center bg-fixed", style:"background-image: url('/background.jpg')",
            div {class:"hidden",
                width: "100%",
                height: "50%",
                background_color: "red",
                "This element is {WINDOW_DIMS():?}"
            }
            div {class:"container mx-auto min-h-screen min-w-screen",
            br {}

                Router::<Route> {},

                br {}
                br {}
                {Footer()}
            }
        }

    )
}

#[component]
fn HomePage() -> Element {
    rsx! {
        {HomePage_slice()}
    }
}

#[component]
fn MinerPage(address: String) -> Element {
    rsx! {
        {MinerPage_slice(address.clone())}
    }
}

#[component]
fn NavBar() -> Element {
    let mut address = use_signal(|| "".to_string());
    let navigator = use_navigator();
    let mut dropdown_menu_toggle = use_signal(|| false);
    let mut dropdown_menu_style = use_signal(|| {
        "visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"
    });

    rsx! {

            nav {class:"bg-opacity-10 bg-white backdrop-filter backdrop-blur-md rounded-full shadow-lg space-x-4 py-2 mx-4", id:"navbar-default",
                div {class:"max-w-screen-xl grid grid-cols-3 sm:grid-cols-3 justify-between items-center ps-2",

                    div {class:"sm:hidden col-start-1 col-span-1", h3 {class:"text-slate-200 text-bold", "Sigmanauts"}}

                    button { onclick: move |_| {
                            if dropdown_menu_toggle() {
                                dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear");
                                dropdown_menu_toggle.set(false);
                            }
                            else {
                                dropdown_menu_style.set("visibility: visible; opacity: 1; transition: visibility 1s, opacity 0.2s linear");
                                dropdown_menu_toggle.set(true);
                            }
                        },
                        r#type:"button", class:" sm:hidden col-start-3 inline-flex items-center p-2 me-2 w-10 h-10 justify-center text-sm text-slate-200 rounded-lg hover:bg-slate-100/50 focus:outline-none focus:ring-2 focus:ring-slate-200",
                            svg {class:"h-6 w-6", fill:"none", stroke:"currentColor", "viewBox":"0 0 24 24", xmlns:"http://www.w3.org/2000/svg",
                                path {"stroke-linecap":"round", "stroke-linejoin":"round", "stroke-width":"2", d:"M4 6h16M4 12h16m-7 6h7"}
                            }
                    }

                    div {
                        div {
                            class: "sm:hidden absolute right-0 z-50 m-4 w-56 origin-top-right bg-opacity-10 bg-white backdrop-filter backdrop-blur-md rounded-lg shadow-lg space-x-4 py-2 justify-end items-center text-center content-center ",
                            style: "{dropdown_menu_style}",
                            id: "dropdown_menu",
                            div { Link {onclick: move |_| {dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear");},to: Route::HomePage {}, class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 m-2 ", "Home"}}

                            div { Link {onclick: move |_| {dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear");},to: Route::BlockPage {}, class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 m-2 ", "Blocks"}}

                            div { Link {onclick: move |_| {dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear");},to:"https://discord.com/channels/668903786361651200/1153460448214122526", class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 m-2 ", "Support"}}

                            div { Link {onclick: move |_| {dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear");},to:"https://explorer.ergoplatform.com/payment-request?address=9fFzKA2WHNYyXZWc4MHPtSv6YqS8jtDsZkSnAQwVaAZrYn9ojEA", class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 underline decoration-red-500 m-2 ", "Donate"}}

                            div {
                                form {
                                    onsubmit: move |input| {
                                        navigator.push(Route::MinerPage { address: address() });
                                        dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear");
                                    },
                                    div {
                                        input {
                                            r#type: "text",
                                            class: "bg-white/30 border py-2 px-2 border-slate-300 placeholder-slate-100 focus:outline-none focus:border-slate-500 focus:ring-slate-300 block w-full rounded-full sm:text-sm focus:ring-1",
                                            placeholder: "Enter your mining address",
                                            minlength: "51",
                                            maxlength: "51",
                                            name: "miningaddress",
                                            oninput: move |input| {
                                                address.set(input.value());
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {class:"hidden sm:block sm:grid sm:grid-cols-5 col-start-1 col-end-3 justify-center items-center text-center content-center sm:h-fit sm:w-full sm:mt-2",
                        div { Link {to: Route::HomePage {}, class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 m-2 ", "Home"}
                        }

                        div { Link {to: Route::BlockPage {}, class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 m-2 ", "Blocks"}
                        }

                        div { Link {to:"https://discord.com/channels/668903786361651200/1153460448214122526", class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 m-2 ", "Support"}
                        }

                        div { Link {to:"https://explorer.ergoplatform.com/payment-request?address=9fFzKA2WHNYyXZWc4MHPtSv6YqS8jtDsZkSnAQwVaAZrYn9ojEA", class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 underline decoration-red-500 m-2 ", "Donate"}
                        }

                        div {
                            form {
                                onsubmit: move |input| {
                                    navigator.push(Route::MinerPage { address: address() });
                                },
                                div {
                                    input {
                                        r#type: "text",
                                        class: "bg-white/30 border py-2 px-2 border-slate-300 placeholder-slate-100 focus:outline-none focus:border-slate-500 focus:ring-slate-300 block w-full rounded-full sm:text-sm focus:ring-1",
                                        placeholder: "Enter your mining address",
                                        minlength: "51",
                                        maxlength: "51",
                                        name: "miningaddress",
                                        oninput: move |input| {
                                            address.set(input.value());
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
        }
        if dropdown_menu_toggle(){
            br {}
            br {}
            br {}
            br {}
            br {}
            br {}
        }
        Outlet::<Route> {}
    }
}

#[component]
fn PageNotFound(route: String) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route}" }
    }
}

fn main() {
    #[cfg(feature = "web")]
    tracing_wasm::set_as_global_default();

    #[cfg(feature = "server")]
    tracing_subscriber::fmt::init();

    let debug_flag = true;
    let serve_on_addr: SocketAddr;
    if debug_flag {
        serve_on_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8060);
    } else {
        serve_on_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 13)), 8060);
    }

    // launch_fullstack(app);
    LaunchBuilder::new()
        .with_cfg(server_only! {Config::new().addr(serve_on_addr)})
        .launch(app);
}
