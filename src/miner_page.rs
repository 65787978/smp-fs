use crate::{data::api_data::*, utils::*};
use dioxus::prelude::*;

pub fn MinerPage_slice(address: String) -> Element {
    let address = use_signal(|| String::from(address));
    let data = use_server_future(move || async move { get_data(address()).await })?;

    match &*data.read_unchecked() {
        Some(Ok(stats)) => {
            rsx! {
                {InfoCard("8", "mx-2", shorten_string(address().as_str(), 30).as_str(), "", "Miner Address")}
                    div {class:"grid sm:grid-cols-3 mx-2",
                        {InfoCardDouble("24", "m-1", stats.network.hashrate.to_string().as_str(), "Th/s", "Network Hashrate", stats.pool.hashrate.to_string().as_str(), "Gh/s", "Pool Hashrate")}
                        {InfoCardDouble("24", "m-1", stats.network.reward.to_string().as_str(), "Σ", "Block Reward", stats.network.price.to_string().as_str(), "", "Σ / SigUSD")}
                        {InfoCardDouble("24", "m-1", stats.pool.effort.to_string().as_str(), "%", "Current Pool Effort", stats.miner.round_contribution.to_string().as_str(), "%", "Participation")}
                    }
            }
        }
        Some(Err(error)) => {
            rsx!()
        }
        None => {
            rsx!()
        }
    }
}
