use ordiscan::Ordiscan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api_key = String::from("<YOUR-API-KEY-HERE>");
  let ordiclient = Ordiscan::new(api_key).unwrap();
  let address_activity = ordiclient
    .get_address_activity(String::from("<VALID-BITCOIN-ADDRESS>"))
    .await?;

  println!("{:?}", address_activity.first());
  Ok(())
}
