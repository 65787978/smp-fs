use crate::utils::*;
use dioxus::prelude::*;

pub fn MinerPage_slice(address: String) -> Element {
    rsx! {

            div {class:"grid sm:grid-cols-3 gap-2 m-2",
                {Card("9", "bg-blue-300", address)}
                {InfoCardDouble("8", "bg-gray-300 m-4 text-center", "19.04", "Th/s", "Network Hashrate", "86.55", "Gh/s", "Pool Hashrate")}
                {InfoCardDouble("8", "bg-gray-300 m-4 text-center", "19.04", "Th/s", "Network Hashrate", "86.55", "Gh/s", "Pool Hashrate")}
                // {Card("9", "bg-gray-300", "04".to_string())}
            }
    }
}
