#![allow(unused_variables)]

pub mod schema;
use reqwest::{self, Client};
use std::{collections::HashMap, time::Duration};

static API_BASE_URL: &str = "https://ordiscan.com/v1";

pub struct Ordiscan {
  client: reqwest::Client,
  api_key: String,
}

impl Ordiscan {
  // create a new Ordiscan client
  pub fn new(key: String) -> reqwest::Result<Self> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let api_key: String = key;
    Ok(Self { client, api_key })
  }

  // get inscription info
  // https://ordiscan.com/docs/api#get-inscription-info
  pub fn get_inspection_info(id: Option<String>, number: Option<usize>) {}

  // get a list of inscriptions
  // https://ordiscan.com/docs/api#get-list-of-inscriptions
  pub fn get_list_of_inscriptions(
    address: Option<String>,
    content_type: Option<String>,
    sort: Option<String>,
    after_number: Option<usize>,
    before_number: Option<usize>,
  ) {
  }

  // get address activity
  // https://ordiscan.com/docs/api#get-address-activity
  pub async fn get_address_activity(
    &self,
    address: String,
  ) -> Result<Vec<schema::AddressActivity>, anyhow::Error> {
    let url = format!("{}/activity?address={}", API_BASE_URL, address);
    let header = format!("Bearer {}", self.api_key);

    // get the data
    let data = self
      .client
      .get(&url)
      .header("Authorization", &header)
      .send()
      .await?
      .json::<HashMap<String, Vec<schema::AddressActivity>>>()
      .await?;

    Ok(data.get("data").unwrap().to_vec())
  }
}
