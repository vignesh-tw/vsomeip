use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jobs {
    pub next_page_token: Option<serde_json::Value>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub metrics: Metrics,
    pub window_start: String,
    pub window_end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub total_runs: u64,
    pub failed_runs: u64,
    pub successful_runs: u64,
    pub median_credits_used: u64,
    pub duration_metrics: DurationMetrics,
    pub success_rate: f64,
    pub total_credits_used: u64,
    pub throughput: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationMetrics {
    pub min: u64,
    pub mean: u64,
    pub median: u64,
    pub p95: u64,
    pub max: u64,
    pub standard_deviation: f64,
    pub total_duration: u64,
}