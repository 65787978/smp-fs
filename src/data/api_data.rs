use dioxus::prelude::*;

use crate::server_fn::response::BrowserMockRes;
use chrono::{DateTime, Timelike, Utc};
use reqwest::{self, Client};
use serde::Deserialize;
use server_fn::codec::FromRes;
use server_fn::error::NoCustomError;
use std::collections::VecDeque;
use std::fmt;

use super::structs::*;

const POOL_API_URL: &str = "http://15.204.211.130:4000/api/pools/ErgoSigmanauts";
const NETWORK_API_URL: &str = "https://api.ergoplatform.com/info";
const PRICE_API_URL: &str = "https://api.spectrum.fi/v1/price-tracking/cmc/markets";

/* Display impl */
impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl fmt::Display for MinerStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[server]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok(reqwest::get("https://httpbin.org/ip").await?.text().await?)
}

/// Get data from Mining Core API
#[server]
pub async fn get_data(address: String) -> Result<Stats, ServerFnError> {
    let start_timestamp = Utc::now().timestamp_millis();
    let mut network_stats = NetworkStats::default().await;
    let mut pool_stats = PoolStats::default().await;
    let mut miner_stats = MinerStats::default(address).await;

    network_stats.provide_data().await.unwrap();
    pool_stats.provide_data().await.unwrap();
    miner_stats.provide_data().await.unwrap();

    //participation
    miner_stats.round_contribution =
        ((miner_stats.hashrate_current / (pool_stats.hashrate * 1_000.0)) * 10000.0).round()
            / 100.0;

    let end_timestamp = Utc::now().timestamp_millis();
    println!(
        "GET_DATA Time taken: {} ms",
        end_timestamp - start_timestamp
    );
    Ok(Stats {
        network: network_stats,
        pool: pool_stats,
        miner: miner_stats,
    })
}

/// Get data from Mining Core API
#[server]
pub async fn get_home_page_data() -> Result<Stats, ServerFnError> {
    let start_timestamp = Utc::now().timestamp_millis();

    let mut network_stats = NetworkStats::default().await;
    let mut pool_stats = PoolStats::default().await;

    network_stats.provide_data().await.unwrap();
    pool_stats.provide_data().await.unwrap();

    let end_timestamp = Utc::now().timestamp_millis();
    println!(
        "GET_HOME_PAGE_DATA Time taken: {} ms",
        end_timestamp - start_timestamp
    );

    Ok(Stats {
        network: network_stats,
        pool: pool_stats,
        miner: MinerStats::default("".to_string()).await,
    })
}

/// Get block data
#[server]
pub async fn get_block_data() -> Result<VecBlock, ServerFnError> {
    let start_timestamp = Utc::now().timestamp_millis();
    let mut block_data = VecBlock::default().await;

    block_data.get_block_data().await.unwrap();

    let end_timestamp = Utc::now().timestamp_millis();
    println!(
        "GET_BLOCK_DATA Time taken: {} ms",
        end_timestamp - start_timestamp
    );

    Ok(block_data)
}

impl NetworkStats {
    pub async fn default() -> Self {
        NetworkStats {
            hashrate: f64::default(),
            difficulty: f64::default(),
            height: u64::default(),
            reward: f64::default(),
            reward_reduction: u8::default(),
            price: f64::default(),
        }
    }

    pub async fn provide_data(&mut self) -> Result<(), reqwest::Error> {
        let start_timestamp = Utc::now().timestamp_millis();
        let hashrate_data: serde_json::Value = Client::new()
            .get(NETWORK_API_URL)
            .send()
            .await?
            .json()
            .await?;

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "hashrate_data(NETWORK_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        let info_data: serde_json::Value =
            Client::new().get(POOL_API_URL).send().await?.json().await?;

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "info_data(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        let price_data: serde_json::Value = Client::new()
            .get(PRICE_API_URL)
            .send()
            .await?
            .json()
            .await?;

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "price_data(PRICE_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        self.hashrate = (hashrate_data["hashRate"]
            .as_f64()
            .expect(" network hashrate not awailable")
            / 10_000_000_000.0)
            .round()
            / 100.0;

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "self.hashrate Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        self.difficulty = (info_data["pool"]["networkStats"]["networkDifficulty"]
            .as_f64()
            .expect("network diff not avialable")
            / 10_000_000_000_000.0)
            .round()
            / 100.0;

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "self.difficulty Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        self.height = info_data["pool"]["networkStats"]["blockHeight"]
            .as_u64()
            .expect("block height not available");

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "self.height Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        // ERG Price
        if let serde_json::Value::Array(arr) = price_data {
            for obj in arr {
                if let serde_json::Value::Object(o) = obj {
                    if let Some(base_name) = o.get("base_name") {
                        if base_name == "ERG" {
                            if let Some(quote_name) = o.get("quote_name") {
                                if quote_name == "SigUSD" {
                                    if let Some(last_price) = o.get("last_price") {
                                        if let Some(price) = last_price.as_f64() {
                                            self.price = ((1.0 / price) * 100.0).round() / 100.0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "self.price Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        self.get_block_reward().await.unwrap();

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "self.reward Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        Ok(())
    }

    async fn get_block_reward(&mut self) -> Result<(), reqwest::Error> {
        let start_timestamp = Utc::now().timestamp_millis();
        let block_data: serde_json::Value = Client::new()
            .get(format!("{}/blocks", POOL_API_URL))
            .send()
            .await?
            .json()
            .await?;

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "block_data(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );
        self.reward = block_data[1]["reward"].as_f64().unwrap().round();
        Ok(())
    }
}

impl PoolStats {
    pub async fn default() -> PoolStats {
        PoolStats {
            data: serde_json::Value::default(),
            hashrate: f64::default(),
            connected_miners: u64::default(),
            effort: f64::default(),
            total_blocks: u64::default(),
            confirming_new_block: f64::default(),
        }
    }

    pub async fn provide_data(&mut self) -> Result<(), reqwest::Error> {
        let start_timestamp = Utc::now().timestamp_millis();
        self.fetch_data().await.unwrap();
        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "fetch_data(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        self.calculate_data().await.unwrap();
        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "calculate_data(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );
        Ok(())
    }

    async fn fetch_data(&mut self) -> Result<(), reqwest::Error> {
        let start_timestamp = Utc::now().timestamp_millis();
        self.data = Client::new().get(POOL_API_URL).send().await?.json().await?;

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "fetch_data(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );
        Ok(())
    }

    async fn calculate_data(&mut self) -> Result<(), reqwest::Error> {
        /* Hashrate */
        let start_timestamp = Utc::now().timestamp_millis();
        self.hashrate = (self.data["pool"]["poolStats"]["poolHashrate"]
            .as_f64()
            .expect("no pool hashrate available")
            / 10_000_000.0)
            .round()
            / 100.0;

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "self.hashrate(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        /* Connected Miners */
        self.connected_miners = self.data["pool"]["poolStats"]["connectedMiners"]
            .as_u64()
            .expect("connected miners not available");

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "self.connected_miners(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        /* Pool effort */
        self.effort = (self.data["pool"]["poolEffort"]
            .as_f64()
            .expect("pool effort not available")
            * 10000.0)
            .round()
            / 100.0;

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "self.effort(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        /* Total Blocks */
        self.total_blocks = self.data["pool"]["totalBlocks"]
            .as_u64()
            .expect("total blocks data not available");

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "self.total_blocks(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        /* New block confirmation */
        let block_data: serde_json::Value = Client::new()
            .get(format!("{}/blocks", POOL_API_URL))
            .send()
            .await?
            .json()
            .await?;

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "block_data(POOL_API_URL/blocks) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        let pool_block_confirmation: (&str, f64, f64) = (
            block_data[0]["status"].as_str().unwrap(),
            (block_data[0]["confirmationProgress"].as_f64().unwrap()) * 100.0,
            block_data[0]["reward"].as_f64().unwrap().round(),
        );

        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "pool_block_confirmation(POOL_API_URL/blocks) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        if pool_block_confirmation.0 == "pending" {
            self.confirming_new_block = pool_block_confirmation.1;
        } else {
            self.confirming_new_block = 100.0;
        }

        Ok(())
    }
}

impl MinerStats {
    pub async fn default(address: String) -> Self {
        MinerStats {
            data: serde_json::Value::default(),
            address: address,
            hashrate_collection: VecDeque::default(),
            hashrate_current: f64::default(),
            hashrate_6h: f64::default(),
            hashrate_12h: f64::default(),
            hashrate_24h: f64::default(),
            pending_balance: f64::default(),
            pending_shares: f64::default(),
            round_contribution: f64::default(),
            total_paid: f64::default(),
            paid_24h: f64::default(),
            workers_number: u64::default(),
            workers: Vec::default(),
            chart_data: Vec::default(),
        }
    }
    pub async fn provide_data(&mut self) -> Result<(), reqwest::Error> {
        let start_timestamp = Utc::now().timestamp_millis();
        self.fetch_data().await.unwrap();
        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "fetch_data(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        self.calculate_hashrate_collection().await.unwrap();
        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "calculate_hashrate_collection(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        self.calculate_balance_shares_totalpaid_paid24h()
            .await
            .unwrap();
        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "calculate_balance_shares_totalpaid_paid24h(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        self.calculate_workers().await.unwrap();
        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "calculate_workers(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        self.calculate_hashrate().await.unwrap();
        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "calculate_hashrate(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );

        let start_timestamp = Utc::now().timestamp_millis();
        self.calculate_chart_data().await.unwrap();
        let end_timestamp = Utc::now().timestamp_millis();
        println!(
            "calculate_chart_data(POOL_API_URL) Time taken: {} ms",
            end_timestamp - start_timestamp
        );
        Ok(())
    }

    async fn fetch_data(&mut self) -> Result<(), reqwest::Error> {
        let miner_api_url = format!("{}/{}/{}", POOL_API_URL, "miners", self.address);

        self.data = Client::new()
            .get(miner_api_url)
            .send()
            .await?
            .json()
            .await?;
        Ok(())
    }

    async fn calculate_hashrate_collection(&mut self) -> Result<(), reqwest::Error> {
        //hashrate_collection
        if let serde_json::Value::Array(sample_array) = &self.data["performanceSamples"] {
            for sample in sample_array.iter() {
                let time = sample["created"]
                    .as_str()
                    .expect("time for sample not available");

                let date_time: DateTime<Utc> = DateTime::parse_from_rfc3339(time).unwrap().into();
                let hour = date_time.hour();

                let mut hashrate: f64 = 0.0;

                for (_key, value) in sample["workers"].as_object().unwrap() {
                    hashrate += value["hashrate"]
                        .as_f64()
                        .expect("sample hashrate not available");
                }

                hashrate = (hashrate / 10_000.0).round() / 100.0;

                self.hashrate_collection.push_back((hour, hashrate));
            }
        }
        Ok(())
    }

    async fn calculate_balance_shares_totalpaid_paid24h(&mut self) -> Result<(), reqwest::Error> {
        self.pending_balance = (self.data["pendingBalance"]
            .as_f64()
            .expect("pendingBalance not available")
            * 100.0)
            .round()
            / 100.0;

        self.pending_shares = (self.data["pendingShares"]
            .as_f64()
            .expect("pendingShares not available")
            * 100.0)
            .round()
            / 100.0;

        self.total_paid = (self.data["totalPaid"]
            .as_f64()
            .expect("totalPaid not available")
            * 100.0)
            .round()
            / 100.0;

        self.paid_24h = (self.data["todayPaid"]
            .as_f64()
            .expect("todayPaid not available")
            * 100.0)
            .round()
            / 100.0;

        Ok(())
    }

    async fn calculate_workers(&mut self) -> Result<(), reqwest::Error> {
        for (key, value) in self.data["performance"]["workers"].as_object().unwrap() {
            self.workers.push(Worker {
                name: key.to_string(),
                hashrate: ((value["hashrate"]
                    .as_f64()
                    .expect("failed to load worker hashrate")
                    / 1_000_000.0)
                    * 100.0)
                    .round()
                    / 100.0,
                shares_per_second: value["sharesPerSecond"]
                    .as_f64()
                    .expect("failed to load worker shares per second"),
            })
        }
        Ok(())
    }

    async fn calculate_hashrate(&mut self) -> Result<(), reqwest::Error> {
        //Hashrate current
        let mut hashrate = 0.0;
        let mut workers_number = 0;
        for worker in self.workers.iter() {
            hashrate += worker.hashrate;
            workers_number += 1;
        }
        self.workers_number = workers_number;
        self.hashrate_current = (hashrate * 100.0).round() / 100.0;

        //Hashrate 6h/24h
        let mut hashrate = 0.0;
        let mut itter = 0;
        for worker in self.hashrate_collection.iter() {
            hashrate += worker.1;

            match itter {
                5 => self.hashrate_6h = ((hashrate / 6.0) * 100.0).round() / 100.0,
                11 => self.hashrate_12h = ((hashrate / 12.0) * 100.0).round() / 100.0,
                23 => self.hashrate_24h = ((hashrate / 24.0) * 100.0).round() / 100.0,
                _ => {}
            }
            itter += 1;
        }
        Ok(())
    }

    async fn calculate_chart_data(&mut self) -> Result<(), reqwest::Error> {
        for data in self.hashrate_collection.iter() {
            let mut hour = data.0;
            let mut hashrate = data.1;

            self.chart_data
                .push((format!("{hour}:00 "), (format!("{hashrate} Mh/s"))));
        }
        Ok(())
    }
}

impl VecBlock {
    pub async fn default() -> VecBlock {
        VecBlock {
            blocks: vec![Blocks {
                created: String::default(),
                block_height: u64::default(),
                effort: f64::default(),
                effort_avg: f64::default(),
                block_reward: f64::default(),
                confirmation_progress: f64::default(),
                miner: String::default(),
            }],
        }
    }

    async fn get_block_data(&mut self) -> Result<(), reqwest::Error> {
        let data: serde_json::Value = Client::new()
            .get(format!("{}/blocks", POOL_API_URL))
            .send()
            .await?
            .json()
            .await?;

        let mut effort_sum: f64 = 0.0;
        let mut effort_sum_count: f64 = 1.0;

        if let serde_json::Value::Array(block_array) = data {
            for block in block_array {
                if block["reward"].as_f64().unwrap() != 0.0 {
                    effort_sum += (block["effort"].as_f64().unwrap() * 10000.0).round();
                    let effort_avg: f64 = (effort_sum / effort_sum_count).round() / 100.0;
                    effort_sum_count += 1.0;

                    self.blocks.push(Blocks {
                        created: {
                            let date_time: DateTime<Utc> =
                                DateTime::parse_from_rfc3339(block["created"].as_str().unwrap())
                                    .unwrap()
                                    .into();

                            format!("{}", date_time.format("%H:%M  %d/%m/%Y"))
                        },

                        block_height: block["blockHeight"].as_u64().unwrap(),
                        effort: (block["effort"].as_f64().unwrap() * 10000.0).round() / 100.0,
                        effort_avg: effort_avg,
                        block_reward: (block["reward"].as_f64().unwrap() * 100.0).round() / 100.0,
                        confirmation_progress: (block["confirmationProgress"].as_f64().unwrap()
                            * 10000.0)
                            .round()
                            / 100.0,
                        miner: shorten_string(block["miner"].as_str().unwrap(), 15),
                    });
                } else {
                    self.blocks.push(Blocks {
                        created: {
                            let date_time: DateTime<Utc> =
                                DateTime::parse_from_rfc3339(block["created"].as_str().unwrap())
                                    .unwrap()
                                    .into();

                            format!("{}", date_time.format("%H:%M  %d/%m/%Y"))
                        },
                        block_height: block["blockHeight"].as_u64().unwrap(),
                        effort: 0.0,
                        effort_avg: 0.0,
                        block_reward: 0.0,
                        confirmation_progress: 0.0,
                        miner: shorten_string(block["miner"].as_str().unwrap(), 15),
                    });
                }
            }
        }
        Ok(())
    }
}

pub fn shorten_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return String::from(s);
    }

    let start_len = (max_len - 3) / 2;
    let end_len = max_len - 3 - start_len;

    let shortened = format!("{}...{}", &s[..start_len], &s[s.len() - end_len..]);

    shortened
}
