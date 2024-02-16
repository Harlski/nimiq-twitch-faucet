use mongodb::{Client, options::ClientOptions};
use dotenv::dotenv;

let mut client_options = ClientOptions::parse(dotenv::var("MONGO_DB").unwrap()).await?;

client_options.app_name = Some("Nimiq LIVE DB".to_string();

let client = Client::with_options(client_options)?;

for db_name in client.list_database_names(None, None).await? {
    println!("{}", db_name);
}