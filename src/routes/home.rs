use crate::data::api_data::get_data;
use crate::utils::{self, *};
use dioxus::prelude::*;

pub fn HomePage_slice() -> Element {
    let mut data = use_resource(move || async move { get_data("".to_string()).await });

    rsx! {
        div {class:"grid sm:grid-cols-1 justify-items-center",
            div { class:"m-3",
                img {max_width:"15rem", max_height:"15rem", src: "sig-logo.svg"}
            }

            div {class:"grid sm:grid-cols-1 justify-items-center",

                div {class:"text-center text-slate-200 rounded-lg bg-opacity-15 bg-gray backdrop-filter backdrop-blur-md shadow-lg m-2",

                    div {class:"font-mono text-2xl text-slate-200 font-bold", "SIGMANAUTS"}

                    div {class:"text-lg text-center text-slate-200 underline decoration-orange-500", "A community to empower users of the Ergo blockchain"}

                    {ParagraphCard(utils::ParagraphCardProps { vars: ParagraphCard {

                        classes: "bg-inherit m-4 text-center text-slate-300 text-xl".to_string(),
                        text: "Welcome to the Sigmanauts pool, a DAO-driven, community-run mining pool dedicated to supporting the Ergo ecosystem. Joining us not only contributes to the Ergo community (fees go to Sigmanauts treasury) but also offers hourly bonus token payments".to_string(),
                    }})}
                }

            }
        }

    match &*data.read_unchecked() {
        Some(Ok(stats)) => {
            rsx! {
                div {class:"grid sm:grid-cols-4",

                    {InfoCard(utils::InfoCardProps { vars: InfoCard {
                        classes: "".to_string(),
                        value: stats.network.hashrate.to_string(),
                        unit: "Th/s".to_string(),
                        heading: "Network Hashrate".to_string()

                    } })}

                    {InfoCard(utils::InfoCardProps { vars: InfoCard {
                        classes: "".to_string(),
                        value: stats.network.height.to_string(),
                        unit: "".to_string(),
                        heading: "Network Height".to_string()

                    } })}

                    {InfoCard(utils::InfoCardProps { vars: InfoCard {
                        classes: "".to_string(),
                        value: stats.pool.hashrate.to_string(),
                        unit: "Gh/s".to_string(),
                        heading: "Pool Hashrate".to_string()

                    } })}

                    {InfoCard(utils::InfoCardProps { vars: InfoCard {
                        classes: "".to_string(),
                        value: stats.pool.connected_miners.to_string(),
                        unit: "".to_string(),
                        heading: "Pool Miners".to_string()

                    } })}
                }

                div {class:"grid sm:grid-cols-2",

                    {InfoCard(utils::InfoCardProps { vars: InfoCard {
                        classes: "".to_string(),
                        value: "pool.ergo-sig-mining.net:3053".to_string(),
                        unit: "".to_string(),
                        heading: "URL under 10 Gh/s".to_string()

                    } })}


                    {InfoCard(utils::InfoCardProps { vars: InfoCard {
                        classes: "".to_string(),
                        value: "pool.ergo-sig-mining.net:3055".to_string(),
                        unit: "".to_string(),
                        heading: "URL over 10 Gh/s".to_string()

                    } })}

                }

            }
        }
        Some(Err(err)) => {
            rsx! { h1 {"{err}"}}
        }
        None => {
            rsx! {
                    div {class:"grid sm:grid-cols-4",

                        {InfoCard(utils::InfoCardProps { vars: InfoCard {
                            classes: "".to_string(),
                            value: "--".to_string(),
                            unit: "".to_string(),
                            heading: "Network Hashrate".to_string()

                        } })}

                        {InfoCard(utils::InfoCardProps { vars: InfoCard {
                            classes: "".to_string(),
                            value: "--".to_string(),
                            unit: "".to_string(),
                            heading: "Network Height".to_string()

                        } })}

                        {InfoCard(utils::InfoCardProps { vars: InfoCard {
                            classes: "".to_string(),
                            value: "--".to_string(),
                            unit: "".to_string(),
                            heading: "Pool Hashrate".to_string()

                        } })}

                        {InfoCard(utils::InfoCardProps { vars: InfoCard {
                            classes: "".to_string(),
                            value: "--".to_string(),
                            unit: "".to_string(),
                            heading: "Pool Miners".to_string()

                        } })}
                    }

                    div {class:"grid sm:grid-cols-2",

                        {InfoCard(utils::InfoCardProps { vars: InfoCard {
                            classes: "".to_string(),
                            value: "pool.ergo-sig-mining.net:3053".to_string(),
                            unit: "".to_string(),
                            heading: "URL under 10 Gh/s".to_string()

                        } })}


                        {InfoCard(utils::InfoCardProps { vars: InfoCard {
                            classes: "".to_string(),
                            value: "pool.ergo-sig-mining.net:3055".to_string(),
                            unit: "".to_string(),
                            heading: "URL over 10 Gh/s".to_string()

                        } })}

                    }

                }
            }
        }
    }
}
