use log::{debug, error, info, trace, warn};
use std::time::SystemTime;
// use humantime::Rfc3339Timestamp;
use rand::Rng;
use tmi::{Badge};

#[derive(Debug)]
pub struct NlUser {
  pub username: String,
  pub is_subscribed: bool,
}

#[derive(Debug)]
pub struct Eligible {
  pub eligible_users: Vec<NlUser>,
}

impl Eligible {
  pub fn validate_user(&mut self, user: NlUser) {
      let mut user_found = false;

      for eli_user in &self.eligible_users {
          if eli_user.username == user.username {
              println!("\nUser is already in: {}", user.username);
              user_found = true;
              break; 
          }
      }

      if !user_found {
          println!("\nUser is not in the list.");
          self.eligible_users.push(user);
      }
  }

  pub fn list_eligible(&mut self) {
    print!("\nEligible Users:");
    for user in &self.eligible_users{
      print!(" {},", user.username);
    }
  }

  pub fn clear_eligible(&mut self){
    println!("\nClearing Users");
    self.eligible_users.clear();
  }
}

pub fn return_user_struct(msg: &tmi::Privmsg) -> NlUser {
    let user = NlUser {
      username: String::from(msg.sender().name()),
      is_subscribed: msg.badges().any(|badge| matches!(badge, Badge::Subscriber(_))),
    };
    user
  }

pub fn random_number(max: i32) -> i32 {
    // We intended max to be the .len() of the vector/array of users eligible.
    let result = rand::thread_rng().gen_range(0..max);
    result
}

pub fn select_winner(user_list: &Eligible) -> String {
    let winner = &user_list.eligible_users[random_number(user_list.eligible_users.len().try_into().unwrap()) as usize];
    info!("\nWinner was: {} - {:?}\n", &winner.username, user_list);
    winner.username.clone()
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}