use anyhow::Result;
use clap::Parser;
use tokio::select;
use tokio::signal::ctrl_c;
use faucet::{return_user_struct, NlUser, Eligible, EntryCode};
            
#[derive(Parser)]
#[command(author, version)] 
struct Args {
  /// Login username
  #[arg(long)]
  nick: Option<String>,

  /// Login oauth2 token
  #[arg(long)]
  token: Option<String>,

  /// Channels to join
  #[arg(long)]
  channel: Vec<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  // tracing_subscriber::fmt::init();
//     faucet::setup_logger();

  let args = Args::parse();

  let credentials = match args.nick.zip(args.token) {
    Some((nick, token)) => tmi::client::Credentials::new(nick, token),
    None => tmi::client::Credentials::anon(),
  };

  let channels = args
    .channel
    .into_iter()
    .map(tmi::Channel::parse)
    .collect::<Result<Vec<_>, _>>()?;

  println!("Connecting as {}", credentials.nick);
  let mut client = tmi::Client::builder()
    .credentials(credentials)
    .connect()
    .await?;

  client.join_all(&channels).await?;
  println!("Joined the following channels: {}", channels.join(", "));

  select! {
    _ = ctrl_c() => {
      Ok(())
    }
    res = tokio::spawn(run(client, channels)) => {
      res?
    }
  }
}

async fn run(mut client: tmi::Client, channels: Vec<tmi::Channel>) -> Result<()> {
  let mut eligible = Eligible { eligible_users: vec![] };
  let mut entry_code = EntryCode { code: "Start".to_string() };
  loop {
    let msg = client.recv().await?;
    match msg.as_typed()? {

      tmi::Message::Privmsg(msg) => {
        let user = faucet::return_user_struct(&msg);
        eligible.validate_user(user);
        // eligible.list_eligible();
        on_msg(&mut client, msg, &entry_code, &eligible).await?;
        entry_code.generate_new();

      }
      tmi::Message::Reconnect => {
        client.reconnect().await?;
        client.join_all(&channels).await?;
      }
      tmi::Message::Ping(ping) => {
        eligible.clear_eligible();
        client.pong(&ping).await?
      }
      _ => {}
    };
  }
}



async fn on_msg(client: &mut tmi::Client, msg: tmi::Privmsg<'_>, code: &EntryCode, eligible: &Eligible) -> Result<()> {
  println!("\n{}: {}", msg.sender().name(), msg.text());

  if client.credentials().is_anon() {
    return Ok(());
  }

  if msg.text() == code.code {
    let mut send_msg = "";
    let already_eligible = eligible.validate_user(msg.sender().name());
    match already_eligible{
      Some(x) => send_msg = "You are already eligible, no need to re-enter",
      None => send_msg = "You are now eligible! nimiqlSUN"
    }

    client
    .privmsg(msg.channel(), &send_msg)
    .reply_to(msg.message_id())
    .send()
    .await?;
  println!("< {}: {}", msg.channel(), send_msg);
  return Ok(())
}
  if msg.sender().name() != "NimiqLIVE" {
    let send_msg = "nimiqlHEART";
    client
    .privmsg(msg.channel(), &send_msg)
    .reply_to(msg.message_id())
    .send()
    .await?;
  println!("< {}: {}", msg.channel(), send_msg);
  return Ok(())
  }
 

  Ok(())
}

// use log::{info};
// use tmi::{Channel, Client, Message, Badge};
// use anyhow::Result;

// use tokio::main;

// use dotenv::dotenv;
// use std::env;

// use clap::Parser;

// #[derive(Parser)]
// #[command(author, version)]
// struct Args {
//   /// Login username
//   #[arg(long)]
//   nick: Option<String>,

//   /// Login oauth2 token
//   #[arg(long)]
//   token: Option<String>,

//   /// Channels to join
//   #[arg(long)]
//   channel: Vec<String>,
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     dotenv().ok();
//     println!("{}", dotenv::var("MEANING_OF_LIFE").unwrap());

//     let args = Args::parse();

//     let credentials = match args.nick.zip(args.token) {
  //       Some((nick, token)) => tmi::client::Credentials::new(nick, token),
//       None => tmi::client::Credentials::anon(),
//     };


//     println!("Connecting as {}", credentials.nick);
//     let mut client = tmi::Client::builder()
//       .credentials(credentials)
//       .connect()
//       .await?;



//     let channels = vec![
  //         Channel::parse("#nimiqlive".to_string()),
  //     ];
  
  //     let channels: Vec<tmi::Channel> = channels
//     .into_iter()
//     .filter_map(Result::ok)
//     .collect();


//     client.join_all(&channels).await?;
//     println!("Joined the following channels: {}", channels.join(", "));

//     run(&channels).await?;
//     Ok(())
// }



// async fn run(channels: &[tmi::Channel]) -> anyhow::Result<()> {
//   let mut client = tmi::Client::connect().await?;
//   client.join_all(channels).await?;



//   loop {
//     let msg = client.recv().await?;
//     match msg.as_typed()? {
//       tmi::Message::Privmsg(msg) => {


    
//           }
//         }
//       tmi::Message::Reconnect => {
//         client.reconnect().await?;
//         client.join_all(channels).await?;
//       }
//       tmi::Message::Ping(ping) => {
//         if !eligible.eligible_users.is_empty(){
//           faucet::select_winner(&eligible);
//         }
//         eligible.clear_eligible();
//         client.pong(&ping).await?;
//       }
//       _ => {}
//     };
//   }
// } 





