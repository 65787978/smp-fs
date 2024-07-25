use crate::{data::api_data::get_data, utils::*};
use dioxus::prelude::*;

pub fn MinerPage_slice(address: String) -> Element {
    let address = use_signal(|| String::from(address));
    let data = use_server_future(move || async move { get_data(address()).await })?;

    match &*data.read_unchecked() {
        Some(Ok(stats)) => {
            rsx! {
                    div {class:"grid sm:grid-cols-3 gap-2 m-2",
                        {Card("9", "bg-blue-300", address())}
                        {InfoCardDouble("8", "bg-gray-300 m-4 text-center", stats.pool.effort.to_string().as_str(), "%", "Pool Effort", stats.miner.paid_24h.to_string().as_str(), " ERG", "24h Paid")}
                        {InfoCardDouble("8", "bg-gray-300 m-4 text-center", stats.miner.hashrate_current.to_string().as_str(), "Mh/s", "Miner Hashrate", stats.miner.workers_number.to_string().as_str(), "", "Active Workers")}
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
