pub mod schema;
use derive_builder;
use reqwest::{self, Client};
use std::{collections::HashMap, time::Duration};
use url::Url;

static API_BASE_URL: &str = "https://ordiscan.com/v1";

// Define the custom error type
type Result<T> = std::result::Result<T, OrdiscanError>;

#[derive(Debug, thiserror::Error)]
pub enum OrdiscanError {
  #[error("api error")]
  RequestError(#[from] reqwest::Error),
}

pub struct Ordiscan {
  client: reqwest::Client,
  api_key: String,
}

#[derive(derive_builder::Builder, Debug)]
pub struct GetListOfInscriptionParams<'a> {
  pub address: Option<&'a str>,
  pub content_type: Option<&'a str>,
  pub sort: Option<&'a str>,
  pub after_number: Option<usize>,
  pub before_number: Option<usize>,
}

#[derive(derive_builder::Builder, Debug)]
pub struct GetInscriptionInfoParams<'a> {
  pub id: Option<&'a str>,
  pub number: Option<usize>,
}

impl<'a> Ordiscan {
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
    params: GetInscriptionInfoParams<'a>,
  ) -> Result<schema::InscriptionInfo> {
    let header = format!("Bearer {}", self.api_key);
    let mut url = Url::parse(format!("{}/inscription", API_BASE_URL).as_str()).unwrap();

    if params.id.is_none() & params.number.is_none()
      || params.id.is_some() & params.number.is_some()
    {
      panic!("please supply either id or number")
    }

    if params.id.is_some() {
      url.query_pairs_mut().append_pair("id", params.id.unwrap());
    }

    if params.number.is_some() {
      url
        .query_pairs_mut()
        .append_pair("number", params.number.unwrap().to_string().as_str());
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
    params: GetListOfInscriptionParams<'a>,
  ) -> Result<Vec<schema::InscriptionInfo>> {
    let header = format!("Bearer {}", self.api_key);
    let sort = params.sort.unwrap_or("inscription_number_desc");
    let mut url =
      Url::parse(format!("{}/inscriptions?sort={}", API_BASE_URL, sort).as_str()).unwrap();

    // TODO make this look better
    // dynamically create query params
    if params.address.is_some() {
      url
        .query_pairs_mut()
        .append_pair("address", params.address.unwrap());
    }
    if params.content_type.is_some() {
      url
        .query_pairs_mut()
        .append_pair("content_type", params.content_type.unwrap());
    }
    if params.after_number.is_some() {
      url.query_pairs_mut().append_pair(
        "afterNumber",
        params.after_number.unwrap().to_string().as_str(),
      );
    }
    if params.before_number.is_some() {
      url.query_pairs_mut().append_pair(
        "beforeNumber",
        params.before_number.unwrap().to_string().as_str(),
      );
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
  pub async fn get_address_activity(&self, address: &str) -> Result<Vec<schema::AddressActivity>> {
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
