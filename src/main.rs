//! Run with:
//!
//! ```sh
//! dx serve --platform fullstack
//! ```

#![allow(non_snake_case, unused)]

mod block_page;
mod miner_page;
mod utils;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::block_page::BlockPage;
use crate::miner_page::MinerPage_slice;
use crate::utils::*;
use dioxus::prelude::LaunchBuilder;
use dioxus::prelude::*;
use dioxus_fullstack::Config;
use serde::{Deserialize, Serialize};

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
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
    // let mut count = use_signal(|| 0);
    // let mut text = use_signal(|| "...".to_string());
    // let server_future = use_server_future(get_server_data)?;

    // rsx! {
    //     h1 { "High-Five counter: {count}" }
    //     button { onclick: move |_| count += 1, "Up high!" }
    //     button { onclick: move |_| count -= 1, "Down low!" }
    //     button {
    //         onclick: move |_| async move {
    //             if let Ok(data) = get_server_data().await {
    //                 println!("Client received: {}", data);
    //                 text.set(data.clone());
    //                 post_server_data(data).await.unwrap();
    //             }
    //         },
    //         "Run a server function!"
    //     }
    //     "Server said: {text}"
    // }

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
    // let mut data = use_server_future(move || get_data());
    rsx! {
        div {class:"grid sm:grid-cols-1 justify-items-center",
            div { class:"m-3",
                img {max_width:"10rem", max_height:"10rem",src: "sig-logo.png"}
            }

            div {class:"font-mono text-2xl font-bold", "SIGMANAUTS"}

            div {class:"text-lg text-center underline decoration-orange-500", "A community to empower users of the Ergo blockchain"}


            {Card("11", "bg-inherit m-4 text-center text-xl", "Welcome to the Sigmanauts pool, a DAO-driven, community-run mining pool dedicated to supporting the Ergo ecosystem. Joining us not only contributes to the Ergo community (fees go to Sigmanauts treasury) but also offers hourly bonus token payments.".to_string())},

        }

        div {class:"grid sm:grid-cols-4",
            {InfoCard("8", "bg-gray-300 m-4 text-center", "19.04", "Th/s", "Network Hashrate")}
            {InfoCard("8", "bg-gray-300 m-4 text-center", "1308632", "", "Network Height")}
            {InfoCard("8", "bg-gray-300 m-4 text-center", "65", "Gh/s", "Pool Hashrate")}
            {InfoCard("8", "bg-gray-300 m-4 text-center", "394", "", "Pool Miners")}
        }

        div {class:"grid sm:grid-cols-2",
            {InfoCard("8", "bg-gray-300 m-4 text-center", "URL: pool.ergo-sig-mining.net:3053", "", "Under 10 Gh/s")}
            {InfoCard("8", "bg-gray-300 m-4 text-center", "URL: pool.ergo-sig-mining.net:3055", "", "Over 10 Gh/s")}
        }

        br{}
        br{}
        br{}

    }
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

    let (mut landing_page_ring, mut block_page_ring) = ("", "");

    match current_page_home() {
        "Home" => landing_page_ring = "border border-gray-500 ring-1 ring-gray-500",
        "Blocks" => block_page_ring = "border border-gray-500 ring-1 ring-gray-500",
        _ => {
            landing_page_ring = "";
            block_page_ring = "";
        }
    };

    rsx! {
            nav {class:"bg-opacity-10 bg-white backdrop-filter backdrop-blur-md rounded-lg shadow-lg space-x-4 py-2",
                div {class:"max-w-screen-xl flex flex-wrap mx-auto ps-2",
                    button { "data-collapse-toggle":"navbar-default", "type":"button", "class":"transition duration-300 ease-in-out inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600", "aria-controls":"navbar-default", "aria-expanded":"false",
                        svg {class:"h-6 w-6", fill:"none", stroke:"currentColor", "viewBox":"0 0 24 24", xmlns:"http://www.w3.org/2000/svg",
                            path {"stroke-linecap":"round", "stroke-linejoin":"round", "stroke-width":"2", d:"M4 6h16M4 12h16m-7 6h7"}
                        }
                    }
                    div {class:"hidden w-full md:block md:flex md:flex-row md:justify-center space-x-4",
                        a {onclick: move |_| {navigator.push(Route::LandingPage {}); current_page_home.set("Home")}, class:"font-bold px-3 py-2 text-slate-700 {landing_page_ring} rounded-lg hover:bg-teal-200/50 hover:text-slate-900", "Home"}
                        a {onclick: move |_| {navigator.push(Route::BlockPage {}); current_page_home.set("Blocks")}, class:"font-bold px-3 py-2 text-slate-700 {block_page_ring} rounded-lg hover:bg-teal-200/50 hover:text-slate-900", "Blocks"}
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

#[server]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok(reqwest::get("https://httpbin.org/ip").await?.text().await?)
}

fn main() {
    #[cfg(feature = "web")]
    tracing_wasm::set_as_global_default();

    #[cfg(feature = "server")]
    tracing_subscriber::fmt::init();

    let serve_on_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 13)), 8060);
    // launch_fullstack(app);
    LaunchBuilder::new()
        .with_cfg(server_only! {Config::new().addr(serve_on_addr)})
        .launch(app);
}
