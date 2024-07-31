use crate::{data::api_data::*, utils, InfoCard};
use dioxus::prelude::*;
use gloo::timers::future::TimeoutFuture;
use utils::*;

pub fn MinerPage_slice(address: String) -> Element {
    let address = use_signal(|| String::from(address));
    let mut data = use_server_future(move || async move { get_data(address()).await })?;

    /* Auto update data in background */
    use_future(move || async move {
        loop {
            TimeoutFuture::new(60000).await;
            data.restart();
        }
    });

    match &*data.read_unchecked() {
        Some(Ok(stats)) => {
            rsx! {
                {InfoCard(utils::InfoCardProps { vars: InfoCard {
                    classes: "mx-2 mb-4 mt-4".to_string(),
                    value: shorten_string(address().as_str(), 25),
                    unit: "".to_string(),
                    heading: "Miner Address".to_string()

                } })}

                    div {class:"grid sm:grid-cols-3",

                        {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                            classes: "".to_string(),
                            value_1: stats.network.hashrate.to_string(),
                            unit_1: "Th/s".to_string(),
                            heading_1: "Network Hashrate".to_string(),
                            value_2: stats.pool.hashrate.to_string(),
                            unit_2: "Gh/s".to_string(),
                            heading_2: "Pool Hashrate".to_string(),
                            explanation_bubble: false,
                            bubble_text: "".to_string(),
                        }})}

                        {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                            classes: "".to_string(),
                            value_1: stats.network.reward.to_string(),
                            unit_1: "Σ".to_string(),
                            heading_1: "Block Reward".to_string(),
                            value_2: stats.network.price.to_string(),
                            unit_2: "".to_string(),
                            heading_2: "Σ / SigUSD".to_string(),
                            explanation_bubble: false,
                            bubble_text: "".to_string(),
                        }})}

                        {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                            classes: "".to_string(),
                            value_1: stats.pool.effort.to_string(),
                            unit_1: "%".to_string(),
                            heading_1: "Current Pool Effort".to_string(),
                            value_2: stats.miner.round_contribution.to_string(),
                            unit_2: "%".to_string(),
                            heading_2: "Participation".to_string(),
                            explanation_bubble: false,
                            bubble_text: "".to_string(),
                        }})}
                    }
                    div {class:"grid sm:grid-cols-3",

                    {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                        classes: "".to_string(),
                        value_1: stats.miner.pending_shares.to_string(),
                        unit_1: "".to_string(),
                        heading_1: "Pending Shares".to_string(),
                        value_2: stats.miner.workers_number.to_string(),
                        unit_2: "".to_string(),
                        heading_2: "Active Workers".to_string(),
                        explanation_bubble: false,
                        bubble_text: "".to_string(),
                    }})}

                    {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                        classes: "".to_string(),
                        value_1: stats.miner.paid_24h.to_string(),
                        unit_1: "Σ".to_string(),
                        heading_1: "24h Paid".to_string(),
                        value_2: stats.miner.total_paid.to_string(),
                        unit_2: "Σ".to_string(),
                        heading_2: "Total Paid".to_string(),
                        explanation_bubble: false,
                        bubble_text: "".to_string(),
                    }})}


                    {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                        classes: "".to_string(),
                        value_1: stats.miner.hashrate_current.to_string(),
                        unit_1: "Mh/s".to_string(),
                        heading_1: "Current Hashrate".to_string(),
                        value_2: stats.miner.hashrate_24h.to_string(),
                        unit_2: "Mh/s".to_string(),
                        heading_2: "24h Average".to_string(),
                        explanation_bubble: false,
                        bubble_text: "".to_string(),
                    }})}

                    }
            }
        }
        Some(Err(error)) => {
            rsx!()
        }
        None => {
            rsx! {h1 {"Loading..."}
            }
        }
    }
}
