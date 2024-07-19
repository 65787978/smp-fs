use crate::utils::*;
use dioxus::prelude::*;

pub fn MinerPage_slice(address: String) -> Element {
    rsx! {

            div {class:"grid sm:grid-cols-4 gap-4 m-4",
                {Card("9", "bg-blue-300", address)}
                {Card("9", "bg-gray-300", "02".to_string())}
                {Card("9", "bg-gray-300", "03".to_string())}
                {Card("9", "bg-gray-300", "04".to_string())}
            }
    }
}
