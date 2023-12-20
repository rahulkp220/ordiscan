use ordiscan::Ordiscan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api_key = String::from("<API-KEY-HERE>");
  let ordiclient = Ordiscan::new(api_key).unwrap();

  // get address acitivity
  let address_activity = ordiclient
    .get_address_activity("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh")
    .await?;

  // get inscription info
  let inspection_info = ordiclient
    .get_inscription_info(
      Some("b183b76a2635d1937a60e3eb12e868a64e5fff5e56819cb348cd442877bf95e7i0"),
      None,
    )
    .await?;

  // get list of inscription info
  let list_of_inscriptions = ordiclient
    .get_list_of_inscriptions(
      Some("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"),
      None,
      None,
      None,
      None,
    )
    .await?;

  println!("{:#?}", address_activity);
  println!("{:#?}", inspection_info);
  println!("{:#?}", list_of_inscriptions);

  Ok(())
}
