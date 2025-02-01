// src/config/mod.rs
use serde::Deserialize;
use std::fs;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct ServiceAccount {
    pub project_id: String,
    pub private_key: String,
    pub client_email: String,
}

pub fn load_service_account(path: &str) -> Result<ServiceAccount> {
    let content = fs::read_to_string(path)?;
    let service_account: ServiceAccount = serde_json::from_str(&content)?;
    Ok(service_account)
}