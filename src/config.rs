use serde::Deserialize;
use std::{collections::HashSet, time::Duration};

#[inline]
fn default_down() -> bool {
    true
}

#[inline]
fn default_timeout() -> Duration {
    Duration::from_secs(3)
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub devices: HashSet<String>,
    pub urls: HashSet<String>,
    #[serde(default)]
    pub no_verify: bool,
    #[serde(default = "default_down")]
    pub down: bool,
    #[serde(default)]
    pub up: bool,
    #[serde(default)]
    pub ignore_keys: HashSet<u16>,
    #[serde(default = "default_timeout")]
    pub timeout: Duration,
}
