use log::{info};
use tmi::{Channel, Client, Message};
use anyhow::Result;

fn main() {
    faucet::setup_logger();
    let channels = Channel ("nimiqlive");
    run(channels);
    let mut eligible_users: Vec<String> = vec![];
    // Just some logic to generate random sized arrays. 
    let mut eligible_amount = faucet::random_number(100);
    while eligible_amount > 0 {
        eligible_users.push("User_".to_string() + &eligible_amount.to_string());
        eligible_amount -= 1;
    }
    //
    if eligible_users.len() == 0 { println!("No eligible users."); info!("No eligible users."); }
    else { println!("Winner: {}", faucet::select_winner(eligible_users)); }

}

async fn run(channels: &[tmi::Channel]) -> anyhow::Result<()> {
    let mut client = tmi::Client::connect().await?;
    client.join_all(channels).await?;
  
    loop {
      let msg = client.recv().await?;
      match msg.as_typed()? {
        tmi::Message::Privmsg(msg) => {
          println!("{}: {}", msg.sender().name(), msg.text());
        }
        tmi::Message::Reconnect => {
          client.reconnect().await?;
          client.join_all(channels).await?;
        }
        tmi::Message::Ping(ping) => {
          client.pong(&ping).await?;
        }
        _ => {}
      };
    }
  }