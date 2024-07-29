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

use crate::utils::*;
use data::api_data;
use routes::blocks::BlockPage;
use routes::home::HomePage;
use routes::miner::MinerPage_slice;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use dioxus::prelude::LaunchBuilder;
use dioxus::prelude::*;
use dioxus_fullstack::Config;
use serde::{Deserialize, Serialize};

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

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
    rsx! {
        div {class:"bg-gradient-to-b from-teal-700 to-blue-950 fixed top-0 left-0 right-0 bottom-0 overflow-y-auto",
            div {class:"xl:container mx-auto h-full",
                Router::<Route> {},

                br {}
                br {}
                {Footer()}
            }
        }
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
    let mut small_nav = use_signal(|| "");

    rsx! {

            nav {class:"bg-opacity-10 bg-white backdrop-filter backdrop-blur-md rounded-lg shadow-lg space-x-4 py-2", id:"navbar-default",
                div {class:"max-w-screen-xl flex flex-wrap mx-auto ps-2",
                    button { onclick: move |_| {
                        if small_nav() != "hidden"{
                            small_nav.set("hidden");
                        }
                        else {
                            small_nav.set("");
                        }
                    },"type":"button", "class":"inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-slate-200 rounded-lg md:hidden hover:bg-slate-100/50 focus:outline-none focus:ring-2 focus:ring-slate-200",
                        svg {class:"h-6 w-6", fill:"none", stroke:"currentColor", "viewBox":"0 0 24 24", xmlns:"http://www.w3.org/2000/svg",
                            path {"stroke-linecap":"round", "stroke-linejoin":"round", "stroke-width":"2", d:"M4 6h16M4 12h16m-7 6h7"}
                        }
                    }

                    div {class:"{small_nav} grid grid-rows-5 sm:grid-cols-5 justify-center items-center
                    text-center content-center w-full h-fit sm:h-5 sm:w-full mt-4",
                        div { Link {to: Route::HomePage {}, class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 m-2 ", "Home"}
                        }

                        div { Link {to: Route::BlockPage {}, class:"font-bold text-slate-200 rounded-lg hover:text-slate-100m-2 ", "Blocks"}
                        }

                        div { Link {to:"https://discord.com/channels/668903786361651200/1153460448214122526", class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 m-2 ", "Support"}
                        }

                        div { Link {to:"https://explorer.ergoplatform.com/payment-request?address=9fFzKA2WHNYyXZWc4MHPtSv6YqS8jtDsZkSnAQwVaAZrYn9ojEA", class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 underline decoration-red-500 m-2 ", "Donate"}
                        }

                        div { form {role:"search",  action:"/wallet/{address()}",
                                div { class:"",
                                    input { name:"miningaddress", class:"bg-white/30 border py-2 px-2 border-slate-300 placeholder-slate-100 focus:outline-none focus:border-slate-500 focus:ring-slate-300 block w-full rounded-full sm:text-sm focus:ring-1", placeholder:"Enter your mining address", minlength: 51, maxlength: 51, oninput: move |input| address.set(input.value())}
                                }
                            }
                        }
                    }
                }
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

    let debug_flag = false;
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
