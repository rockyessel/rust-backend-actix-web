use mongodb::{options::ClientOptions, Client};
use std::error::Error;

pub async fn connect_to_mongodb() -> Result<Client, Box<dyn Error>> {
    let uri = "mongodb+srv://rr:rr@rrcluster.x8kvi0e.mongodb.net/?retryWrites=true&w=majority";
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}
