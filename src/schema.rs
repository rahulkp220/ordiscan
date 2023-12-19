#![allow(unused)]

use serde::Deserialize;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Deserialize)]
pub struct BRCAction {
  tick: String,
  action: String,
}

#[derive(Debug, Deserialize)]
pub struct InscriptionInfo {
  inscription_id: String,
  inscription_number: usize,
  content_type: String,
  owner_address: String,
  owner_output: String,
  genesis_address: String,
  genesis_output: String,
  timestamp: String,
  content_url: String,
  submodules: Option<String>,
  sats_name: Option<String>,
  brc_action: Option<BRCAction>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddressActivity {
  txid: String,
  r#type: String,
  inscription_id: String,
  counterpart_address: String,
  spent_as_fee: bool,
  timestamp: String,
}
