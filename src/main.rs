// To do:
// I need to create a function that will add user to eligible_users array, confirm they're not already eligble etc.
// I need to create a clear eligible_users function
// I need to organize the flow of adding users to eligibility, otherwise I'll end up putting all my login in the tmi::Message::Privmsg logic. Which is not good..
// Slowly chunging away at this. 

use log::{info};
use tmi::{Channel, Client, Message, Badge};
use anyhow::Result;
use faucet::{return_user_struct, NlUser, Eligible};

use tokio::main;

// fn main() {
//     faucet::setup_logger();

//     let mut eligible_users: Vec<String> = vec![];
//     // Just some logic to generate random sized arrays. 
//     let mut eligible_amount = faucet::random_number(100);
//     while eligible_amount > 0 {
//         eligible_users.push("User_".to_string() + &eligible_amount.to_string());
//         eligible_amount -= 1;
//     }
//     //
//     if eligible_users.len() == 0 { println!("No eligible users."); info!("No eligible users."); }
//     else { println!("Winner: {}", faucet::select_winner(eligible_users)); }

// }

#[tokio::main]
async fn main() -> Result<()> {
    faucet::setup_logger();

    let channels = vec![
        Channel::parse("#nimiqlive".to_string()),
    ];

    let channels: Vec<tmi::Channel> = channels
    .into_iter()
    .filter_map(Result::ok)
    .collect();
  
    run(&channels).await?;
    Ok(())
}


async fn run(channels: &[tmi::Channel]) -> anyhow::Result<()> {
  let mut client = tmi::Client::connect().await?;
  client.join_all(channels).await?;

  let mut eligible = Eligible { eligible_users: vec![] };

  loop {
    let msg = client.recv().await?;
    match msg.as_typed()? {
      tmi::Message::Privmsg(msg) => {
          let user = faucet::return_user_struct(&msg);
          eligible.validate_user(user);
          eligible.list_eligible();
        }
      tmi::Message::Reconnect => {
        client.reconnect().await?;
        client.join_all(channels).await?;
      }
      tmi::Message::Ping(ping) => {
        if !eligible.eligible_users.is_empty(){
          faucet::select_winner(&eligible);
        }
        eligible.clear_eligible();
        client.pong(&ping).await?;
      }
      _ => {}
    };
  }
} 




