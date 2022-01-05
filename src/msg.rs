use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::viewing_key::ViewingKey;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub max_size: i32,
    pub prng_seed: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Record { reminder: String },
    Read {},
    GenerateViewingKey {entropy: String, padding: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Stats {},
    Read { address: HumanAddr, key: String, }
}

impl QueryMsg {
    pub fn get_validation_params(&self) -> (Vec<&HumanAddr>, ViewingKey) {
        match self {
                Self::Read { address, key, .. } => (vec![address], ViewingKey(key.clone())),
                _ => panic!("This query type does not require authentication"),
        }
    }
}

/// Responses from handle function
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    Record  { 
        status: String
     },
    Read {
        status: String, reminder: Option<String>, timestamp: Option<u64> 
    },
    GenerateViewingKey { 
        key: ViewingKey 
    },
}

/// Responses from query function
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    Stats {
        reminder_count: u64,
    },
    Read {
        status: String,
        reminder: Option<String>,
        timestamp: Option<u64>,
    }
}