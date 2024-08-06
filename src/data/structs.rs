use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Worker {
    pub name: String,
    pub hashrate: f64,
    pub shares_per_second: f64,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkStats {
    pub hashrate: f64,
    pub difficulty: f64,
    pub height: u64,
    pub reward: f64,
    pub reward_reduction: u8,
    pub price: f64,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoolStats {
    pub data: serde_json::Value,
    pub hashrate: f64,
    pub connected_miners: u64,
    pub effort: f64,
    pub total_blocks: u64,
    pub confirming_new_block: f64,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinerStats {
    pub data: serde_json::Value,
    pub address: String,
    pub hashrate_collection: VecDeque<(u32, f64)>,
    pub hashrate_current: f64,
    pub hashrate_6h: f64,
    pub hashrate_12h: f64,
    pub hashrate_24h: f64,
    pub pending_shares: f64,
    pub pending_balance: f64,
    pub round_contribution: f64,
    pub total_paid: f64,
    pub paid_24h: f64,
    pub workers_number: u64,
    pub workers: Vec<Worker>,
    pub chart_data: Vec<(String, f32)>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Stats {
    pub network: NetworkStats,
    pub pool: PoolStats,
    pub miner: MinerStats,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Blocks {
    pub created: String,
    pub block_height: u64,
    pub effort: f64,
    pub effort_avg: f64,
    pub block_reward: f64,
    pub confirmation_progress: f64,
    pub miner: String,
}
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct VecBlock {
    pub blocks: Vec<Blocks>,
}
