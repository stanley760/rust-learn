use std::{fmt::Display, sync::Arc};

use anyhow::Result;
use dashmap::DashMap;

#[derive(Debug,Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&mut self, key: impl Into<String>) -> Result<()> {
       
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}