use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct BRCAction {
  tick: String,
  amount: usize,
  action: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
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
  submodules: Option<Vec<String>>,
  sats_name: Option<String>,
  brc_action: Option<BRCAction>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct AddressActivity {
  txid: String,
  r#type: String,
  inscription_id: String,
  counterpart_address: String,
  spent_as_fee: bool,
  timestamp: String,
}
