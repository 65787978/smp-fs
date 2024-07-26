use crate::utils::*;
use dioxus::prelude::*;

pub fn BlockPage() -> Element {
    rsx! {

        div {class:"grid sm:grid-cols-4 gap-4 m-4",
            {Card("11", "bg-gray-300", "01".to_string())}
            {Card("11", "bg-gray-300", "02".to_string())}
            {Card("11", "bg-gray-300", "03".to_string())}
            {Card("11", "bg-gray-300", "04".to_string())}

        }

        div {class:"card min-h-[30rem] rounded-lg shadow bg-gray-700 m-4", "Block Table"}

    }
}
