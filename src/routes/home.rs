use crate::data::api_data::get_landing_page_data;
use crate::utils::*;
use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    // let mut data = use_server_future(move || get_data());
    let mut data =
        use_server_future(move || async move { get_landing_page_data("".to_string()).await })?;

    match &*data.read_unchecked() {
        Some(Ok(stats)) => {
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
                    {InfoCard("8", "", stats.network.hashrate.to_string().as_str(), "Th/s", "Network Hashrate")}
                    {InfoCard("8", "", stats.network.height.to_string().as_str(), "", "Network Height")}
                    {InfoCard("8", "", stats.pool.hashrate.to_string().as_str(), "Gh/s", "Pool Hashrate")}
                    {InfoCard("8", "", stats.pool.connected_miners.to_string().as_str(), "", "Pool Miners")}
                }

                div {class:"grid sm:grid-cols-2",
                    {InfoCard("8", "", "pool.ergo-sig-mining.net:3053", "", "URL under 10 Gh/s")}
                    {InfoCard("8", "", "pool.ergo-sig-mining.net:3055", "", "URL over 10 Gh/s")}
                }


            }
        }
        Some(Err(err)) => {
            rsx! { h1 {"{err}"}}
        }
        None => {
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
                    {InfoCardPlaceholder("8", " ")}
                    {InfoCardPlaceholder("8", "")}
                    {InfoCardPlaceholder("8", "")}
                    {InfoCardPlaceholder("8", "")}

                }

                div {class:"grid sm:grid-cols-2",
                    {InfoCardPlaceholder("8", "")}
                    {InfoCardPlaceholder("8", "")}
                }

            }
        }
    }
}
