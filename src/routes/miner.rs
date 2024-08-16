use crate::{data::api_data::*, utils, InfoCard, WINDOW_DIMS};
use charming::{
    component::{Axis, Legend, Type},
    element::{
        AreaStyle, AxisType, Formatter, ItemStyle, Label, LineStyle, OriginPosition, Symbol,
    },
    series::{Line, Pie, PieRoseType, Sankey, SankeyLink},
    Chart, WasmRenderer,
};
use dioxus::prelude::*;
use gloo::timers::future::TimeoutFuture;
use serde::de::IntoDeserializer;
use utils::*;

pub fn MinerPage_slice(address: String) -> Element {
    let address = use_signal(|| String::from(address));
    let mut data = use_resource(move || async move { get_data(address()).await });

    /* Auto update data in background */
    use_future(move || async move {
        loop {
            TimeoutFuture::new(60000).await;
            data.restart();
        }
    });

    spawn(async move {
        let mut hour: Vec<String> = vec![];
        let mut hashrate: Vec<f64> = vec![];

        let chart_data = data.clone();

        match &*chart_data.read_unchecked() {
            Some(Ok(stats)) => {
                for (hour_data, hashrate_data) in &stats.miner.chart_data {
                    hour.push(hour_data.clone());
                    hashrate.push(hashrate_data.clone());
                }

                let chart = Chart::new()
                    .x_axis(Axis::new().type_(AxisType::Category).data(hour))
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(
                        Line::new()
                            .area_style(
                                AreaStyle::new()
                                    .color("#FFE6F7")
                                    .opacity(0.5)
                                    .origin(OriginPosition::Auto),
                            )
                            .line_style(LineStyle::new().color("#FFE6F7").width(3))
                            .smooth(0.5)
                            .show_symbol(false)
                            .item_style(ItemStyle::new().color("#FFE6F7"))
                            .data(hashrate),
                    );

                let chart_width: u32;

                if WINDOW_DIMS().0 < 640.0 {
                    chart_width = (WINDOW_DIMS().0 * 0.96) as u32;
                } else {
                    chart_width = (WINDOW_DIMS().0 * 0.79) as u32;
                }

                let renderer = WasmRenderer::new(chart_width, 400);

                renderer.render("chart", &chart).unwrap();
            }
            Some(Err(error)) => {}
            None => {}
        }
    });

    match &*data.read_unchecked() {
        Some(Ok(stats)) => {
            rsx! {
                    {InfoCard(utils::InfoCardProps { vars: InfoCard {
                        classes: "mx-2 mb-4 mt-4".to_string(),
                        value: {if WINDOW_DIMS().0 < 640.0 {shorten_string(address().as_str(), 25)} else {address().clone()}},
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

                    div {class:"grid sm:grid-col-1 justify-center items-center
                    text-center content-center w-max",
                        div {class:"text-slate-200 rounded-lg bg-opacity-15 bg-gray backdrop-filter backdrop-blur-md shadow-lg m-2",
                            id: "chart",
                        }
                    }

            }
        }
        Some(Err(error)) => {
            rsx!()
        }
        None => {
            rsx! {
                {InfoCard(utils::InfoCardProps { vars: InfoCard {
                    classes: "mx-2 mb-4 mt-4".to_string(),
                    value: {if WINDOW_DIMS().0 < 640.0 {shorten_string(address().as_str(), 25)} else {address().clone()}},
                    unit: "".to_string(),
                    heading: "Miner Address".to_string()

                } })}

                    div {class:"grid sm:grid-cols-3",

                        {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                            classes: "".to_string(),
                            value_1: "--".to_string(),
                            unit_1: "".to_string(),
                            heading_1: "Network Hashrate".to_string(),
                            value_2: "--".to_string(),
                            unit_2: "".to_string(),
                            heading_2: "Pool Hashrate".to_string(),
                            explanation_bubble: false,
                            bubble_text: "".to_string(),
                        }})}

                        {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                            classes: "".to_string(),
                            value_1: "--".to_string(),
                            unit_1: "".to_string(),
                            heading_1: "Block Reward".to_string(),
                            value_2: "--".to_string(),
                            unit_2: "".to_string(),
                            heading_2: "Σ / SigUSD".to_string(),
                            explanation_bubble: false,
                            bubble_text: "".to_string(),
                        }})}

                        {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                            classes: "".to_string(),
                            value_1: "--".to_string(),
                            unit_1: "".to_string(),
                            heading_1: "Current Pool Effort".to_string(),
                            value_2: "--".to_string(),
                            unit_2: "".to_string(),
                            heading_2: "Participation".to_string(),
                            explanation_bubble: false,
                            bubble_text: "".to_string(),
                        }})}
                    }
                    div {class:"grid sm:grid-cols-3",

                    {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                        classes: "".to_string(),
                        value_1: "--".to_string(),
                        unit_1: "".to_string(),
                        heading_1: "Pending Shares".to_string(),
                        value_2: "--".to_string(),
                        unit_2: "".to_string(),
                        heading_2: "Active Workers".to_string(),
                        explanation_bubble: false,
                        bubble_text: "".to_string(),
                    }})}

                    {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                        classes: "".to_string(),
                        value_1: "--".to_string(),
                        unit_1: "".to_string(),
                        heading_1: "24h Paid".to_string(),
                        value_2: "--".to_string(),
                        unit_2: "".to_string(),
                        heading_2: "Total Paid".to_string(),
                        explanation_bubble: false,
                        bubble_text: "".to_string(),
                    }})}


                    {InfoCardDouble(utils::InfoCardDoubleProps {vars: InfoCardDouble {
                        classes: "".to_string(),
                        value_1: "--".to_string(),
                        unit_1: "".to_string(),
                        heading_1: "Current Hashrate".to_string(),
                        value_2: "--".to_string(),
                        unit_2: "".to_string(),
                        heading_2: "24h Average".to_string(),
                        explanation_bubble: false,
                        bubble_text: "".to_string(),
                    }})}

                    }
            }
        }
    }
}
