use crate::{data::api_data::*, utils::*};
use dioxus::prelude::*;

pub fn MinerPage_slice(address: String) -> Element {
    let address = use_signal(|| String::from(address));
    let data = use_server_future(move || async move { get_data(address()).await })?;

    match &*data.read_unchecked() {
        Some(Ok(stats)) => {
            rsx! {
                {InfoCard("8", "mx-2 mb-2 mt-4", address().as_str(), "", "Miner Address")}
                    div {class:"grid sm:grid-cols-3",
                        {InfoCardDouble("8", "m-2", stats.network.hashrate.to_string().as_str(), "Th/s", "Network Hashrate", stats.pool.hashrate.to_string().as_str(), "Gh/s", "Pool Hashrate")}
                        {InfoCardDouble("8", "m-2", stats.network.reward.to_string().as_str(), "Σ", "Block Reward", stats.network.price.to_string().as_str(), "", "Σ / SigUSD")}
                        {InfoCardDouble("8", "m-2", stats.pool.effort.to_string().as_str(), "%", "Current Pool Effort", stats.miner.round_contribution.to_string().as_str(), "%", "Participation")}
                    }
                    div {class:"grid sm:grid-cols-3",
                        {InfoCardDouble("8", "m-2", stats.miner.pending_shares.to_string().as_str(), "", "Pending Shares", stats.miner.workers_number.to_string().as_str(), "", "Active Workers")}
                        {InfoCardDouble("8", "m-2", stats.miner.paid_24h.to_string().as_str(), "Σ", "24h Paid", stats.miner.total_paid.to_string().as_str(), "Σ", "Total Paid")}
                        {InfoCardDouble("8", "m-2", stats.miner.hashrate_current.to_string().as_str(), "Mh/s", "Current Hashrate", stats.miner.hashrate_24h.to_string().as_str(), "Mh/s", "24h Average Hashrate")}
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
