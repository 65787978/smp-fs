use crate::{data::api_data::*, utils::*};
use dioxus::prelude::*;

pub fn BlockPage() -> Element {
    let mut block_data = use_server_future(move || async move { get_block_data().await })?;
    let mut global_data =
        use_server_future(move || async move { get_landing_page_data("".to_string()).await })?;

    rsx! {
        match &*global_data.read_unchecked() {
            Some(Ok(stats)) => {
                rsx!{
                    div {class:"grid sm:grid-cols-4 mt-2",
                        {InfoCard("8", "", stats.network.hashrate.to_string().as_str(), "Th/s", "Network Hashrate")}
                        {InfoCard("8", "", stats.network.difficulty.to_string().as_str(), "P", "Network Difficulty")}
                        {InfoCard("8", "", stats.pool.effort.to_string().as_str(), "%", "Current Pool Effort")}
                        {InfoCard("8", "", stats.pool.connected_miners.to_string().as_str(), "", "Connected Miners")}
                    }
                }
            }
            Some(Err(error)) => rsx!("{error}"),
            None => rsx!("Loading...")
        }

        match &*block_data.read_unchecked() {
            Some(Ok(block_stats)) => {
                rsx!(
                    div {class:"text-center rounded-lg bg-opacity-30 bg-white backdrop-filter backdrop-blur-md shadow-lg m-2 m-2",
                        div {class:"overflow-x-scroll shadow-md sm:rounded-lg",
                            table {class: "w-full text-sm text-center flex-nowrap",
                                    thead { class:"uppercase bg-slate-50/60",
                                        tr{
                                            th{ scope: "col", "CREATED"}
                                            th{ scope: "col", "HEIGHT"}
                                            th{ scope: "col", "EFFORT"}
                                            th{ scope: "col", "REWARD"}
                                            th{ scope: "col", "STATUS"}
                                            th{ scope: "col", "MINER"}
                                        }
                                    }
                                tbody {
                                        for block in block_stats.blocks.iter(){
                                            tr{ class:"hover:bg-slate-50/40",
                                                if block.created != "" {
                                                    td{"{block.created}"}
                                                    td{"{block.block_height}"}

                                                    td{
                                                        div {class:"rounded-lg", style:"height: 1.75rem; width:100%",

                                                            if block.effort == 0.0 {

                                                                div {class:"h-full bg-red-400/50 rounded-lg", style:"width: 100%", b { class:"align-middle", "ORPHAN"}}
                                                            }
                                                            else if block.effort < 100.0 {

                                                                div {class:"h-full bg-green-500/50 rounded-lg", style:"width: 100%", b{class:"align-middle","{block.effort}%"}}
                                                            }
                                                            else if block.effort > 100.0 && block.effort < 200.0 {

                                                                div {class:"h-full bg-yellow-400/50 rounded-lg", style:"width: 100%", b{class:"align-middle","{block.effort}%"}}
                                                            }
                                                            else {
                                                                div {class:"h-full bg-red-500/50 rounded-lg", style:"width: 100%", b{class:"align-middle","{block.effort}%"}}
                                                            }
                                                        }
                                                    }

                                                    td{"{block.block_reward} Σ"}

                                                    if block.confirmation_progress == 0.0 && block.block_reward == 0.0
                                                    {
                                                        td{
                                                            div {class:"w-full bg-gray-300/30 rounded-full", style:"height: 1.75rem;",
                                                                div {class:"h-full bg-red-400/50 rounded-full", style:"width: 100%", b{class:"align-middle","ORPHAN"}}
                                                            }
                                                        }
                                                    }
                                                    else if block.confirmation_progress == 100.0
                                                    {
                                                        td{
                                                            div {class:"w-full bg-gray-300/30 rounded-full", style:"height: 1.75rem;",
                                                                div {class:"h-full bg-cyan-600/50 rounded-full", style:"width: 100%", b{class:"align-middle","Confirmed"}}
                                                            }
                                                        }
                                                    }
                                                    else {
                                                        td{
                                                            div {class:"w-full bg-gray-300/30 rounded-full", style:"height: 1.75rem;",
                                                                div {class:"h-full bg-cyan-300/50 rounded-full", style:"width: {block.confirmation_progress}%", b{class:"align-middle","{block.confirmation_progress}%"}}
                                                            }
                                                        }
                                                    }

                                                    td{"{block.miner}"}
                                                }

                                            }
                                        }
                                    }
                                }
                            }
                    }
                )
            }
            Some(Err(error)) => {
                rsx!("{error")
            }
            None => {
                rsx!("Loading...")
            }
        }
    }
}
