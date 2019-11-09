use crate::{Error, Result};

use serde::{Deserialize, Serialize};

/// Sent from server to client, this shared model is used for all client communication
#[allow(variant_size_differences)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseMessage {
  Hello {
    u: Box<crate::profile::UserProfile>,
    b: bool
  },
  ServerError {
    reason: String,
    content: String
  },
  Pong {
    v: i64
  }
}

impl ResponseMessage {
  pub fn from_json(s: &str) -> Result<ResponseMessage> {
    serde_json::from_str(&s).map_err(|e| Error::from(format!("Can't decode json ResponseMessage: {}", e)))
  }

  pub fn to_json(&self) -> Result<String> {
    serde_json::to_string_pretty(&self).map_err(|e| Error::from(format!("Can't encode json ResponseMessage: {}", e)))
  }

  pub fn from_binary(b: &[u8]) -> Result<ResponseMessage> {
    bincode::deserialize(&b).map_err(|e| Error::from(format!("Can't decode binary ResponseMessage: {}", e)))
  }

  pub fn to_binary(&self) -> Result<Vec<u8>> {
    bincode::serialize(&self).map_err(|e| Error::from(format!("Can't encode binary ResponseMessage: {}", e)))
  }
}