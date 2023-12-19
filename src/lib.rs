#![allow(unused_variables)]

pub mod schema;
use reqwest::{self, Client};
use std::{collections::HashMap, time::Duration};
use url::Url;

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
  pub async fn get_inscription_info(
    &self,
    id: Option<String>,
    number: Option<usize>,
  ) -> Result<schema::InscriptionInfo, anyhow::Error> {
    let header = format!("Bearer {}", self.api_key);
    let mut url = Url::parse(format!("{}/inscription", API_BASE_URL).as_str())?;

    if id.is_some() {
      url
        .query_pairs_mut()
        .append_pair("id", id.unwrap().as_str());
    }

    if number.is_some() {
      url
        .query_pairs_mut()
        .append_pair("number", number.unwrap().to_string().as_str());
    }

    let data = self
      .client
      .get(url.to_string())
      .header("Authorization", &header)
      .send()
      .await?
      .json::<HashMap<String, schema::InscriptionInfo>>()
      .await?;

    Ok(data.get("data").unwrap().to_owned())
  }

  // get a list of inscriptions
  // https://ordiscan.com/docs/api#get-list-of-inscriptions
  pub async fn get_list_of_inscriptions(
    &self,
    address: Option<String>,
    content_type: Option<String>,
    sort: Option<String>,
    after_number: Option<usize>,
    before_number: Option<usize>,
  ) -> Result<Vec<schema::InscriptionInfo>, anyhow::Error> {
    let header = format!("Bearer {}", self.api_key);
    let sort = sort.unwrap_or(String::from("inscription_number_desc"));
    let mut url = Url::parse(format!("{}/inscriptions?sort={}", API_BASE_URL, sort).as_str())?;

    // TODO make this look better
    // dynamically create query params
    if address.is_some() {
      url
        .query_pairs_mut()
        .append_pair("address", address.unwrap().as_str());
    }
    if content_type.is_some() {
      url
        .query_pairs_mut()
        .append_pair("content_type", content_type.unwrap().as_str());
    }
    if after_number.is_some() {
      url
        .query_pairs_mut()
        .append_pair("afterNumber", after_number.unwrap().to_string().as_str());
    }
    if before_number.is_some() {
      url
        .query_pairs_mut()
        .append_pair("beforeNumber", before_number.unwrap().to_string().as_str());
    }

    // get the data
    let data = self
      .client
      .get(url.to_string())
      .header("Authorization", &header)
      .send()
      .await?
      .json::<HashMap<String, Vec<schema::InscriptionInfo>>>()
      .await?;

    Ok(data.get("data").unwrap().to_vec())
  }

  // get address activity
  // https://ordiscan.com/docs/api#get-address-activity
  pub async fn get_address_activity(
    &self,
    address: String,
  ) -> Result<Vec<schema::AddressActivity>, anyhow::Error> {
    let api_url = format!("{}/activity?address={}", API_BASE_URL, address);
    let header = format!("Bearer {}", self.api_key);

    // get the data
    let data = self
      .client
      .get(&api_url)
      .header("Authorization", &header)
      .send()
      .await?
      .json::<HashMap<String, Vec<schema::AddressActivity>>>()
      .await?;

    Ok(data.get("data").unwrap().to_vec())
  }
}
