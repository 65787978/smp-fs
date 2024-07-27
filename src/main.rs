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
mod pages {
    pub mod block_page;
    pub mod landing_page;
    pub mod miner_page;
}

mod utils;

use crate::utils::*;
use data::api_data;
use pages::block_page::BlockPage;
use pages::landing_page::LandingPage_slice;
use pages::miner_page::MinerPage_slice;
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
        LandingPage {},
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
        div {class:"bg-gradient-to-b from-teal-100 to-blue-950 fixed top-0 left-0 right-0 bottom-0 overflow-y-auto",
            div {class:"xl:container mx-auto h-full",
                Router::<Route> {},

                {Footer()}
            }
        }
    }
}

#[component]
fn LandingPage() -> Element {
    rsx!({ LandingPage_slice() })
}

#[component]
fn MinerPage(address: String) -> Element {
    let address = use_signal(|| address);

    rsx! {
        {MinerPage_slice(address())},
    }
}

#[component]
fn NavBar() -> Element {
    let mut address = use_signal(|| "".to_string());
    let navigator = use_navigator();
    let mut current_page_home = use_signal(|| "");

    rsx! {
            nav {class:"bg-opacity-10 bg-white backdrop-filter backdrop-blur-md rounded-lg shadow-lg space-x-4 py-2", id:"navbar-default",
                div {class:"max-w-screen-xl flex flex-wrap mx-auto ps-2",
                    button { "data-collapse-toggle":"navbar-default", "type":"button", "class":"transition duration-300 ease-in-out inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600", "aria-controls":"navbar-default", "aria-expanded":"true",
                        svg {class:"h-6 w-6", fill:"none", stroke:"currentColor", "viewBox":"0 0 24 24", xmlns:"http://www.w3.org/2000/svg",
                            path {"stroke-linecap":"round", "stroke-linejoin":"round", "stroke-width":"2", d:"M4 6h16M4 12h16m-7 6h7"}
                        }
                    }
                    div {class:"hidden w-full md:block md:flex md:flex-row md:justify-center space-x-4",
                        a {onclick: move |_| {navigator.push(Route::LandingPage {}); current_page_home.set("Home")}, class:"font-bold px-3 py-2 text-slate-700 rounded-lg hover:bg-teal-200/50 hover:text-slate-900", "Home"}
                        a {onclick: move |_| {navigator.push(Route::BlockPage {}); current_page_home.set("Blocks")}, class:"font-bold px-3 py-2 text-slate-700 rounded-lg hover:bg-teal-200/50 hover:text-slate-900", "Blocks"}
                        a{href:"/", class:"font-bold px-3 py-2 text-slate-700 rounded-lg hover:bg-teal-200/50 hover:text-slate-900 text-center", "Support"}
                        a{href:"/", class:"font-bold px-3 py-2 text-slate-700 rounded-lg hover:bg-teal-200/50 hover:text-slate-900 text-center underline decoration-red-500", "Donate"}
                        form {role:"search",  action:"/wallet/{address()}",
                            div { class:"",
                                input { name:"miningaddress", class:"px-2 py-3 bg-white/50 border border-slate-300 placeholder-slate-400 focus:outline-none focus:border-gray-500 focus:ring-gray-500 block w-full rounded-full sm:text-sm focus:ring-1", placeholder:"Enter your mining address", minlength: 51, maxlength: 51, oninput: move |input| address.set(input.value())}
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
