use dotenv::dotenv;
use log::{debug, error, info, trace, warn};
use std::time::SystemTime;
// use humantime::Rfc3339Timestamp;
use rand::Rng;
use tmi::{Badge};
use random_str as random;
pub mod db;

#[derive(Debug)]
pub struct EntryCode {
  pub code: String,
}

#[derive(Debug)]
pub struct NlUser {
  pub username: String,
  pub is_subscribed: bool,
}

#[derive(Debug)]
pub struct Eligible {
  pub eligible_users: Vec<NlUser>,
}

impl EntryCode{
  pub fn generate_new(&mut self){
    let length = 4;
    let lowercase = true;
    let uppercase = true;
    let numbers = true;
    let symbols = false;
    self.code = random::get_string(length, lowercase, uppercase, numbers, symbols);
    println!("New code: {}", self.code);
  }
}

impl Eligible {
  // Validate user is used to check to see if a user is already in the eligiblity list
  pub fn validate_user(&mut self, user: NlUser) {
      let mut user_found = false;

      for eli_user in &self.eligible_users {
          if eli_user.username == user.username {
              db::do_something(eli_user);
              println!("User is already in: {}", user.username);
              user_found = true;
              break; 
          }
      }

      if !user_found {
          println!("{:?} is not in the list.", &user.username);
          self.eligible_users.push(user);
      }
  }

  // List eligible in the current list, ex: 'Eligible Users: NimiqLIVE, user1, user2' etc.
  pub fn list_eligible(&mut self) {
    if self.eligible_users.len() == 0 {
      println!("Eligible Users: None");
      return;
    }
    print!("\nEligible Users:");
    for user in &self.eligible_users{
      print!(" {},", user.username);
    }
  }

  // Clears eligibility list.
  pub fn clear_eligible(&mut self){
    println!("\nClearing Users");
    self.list_eligible();
    self.eligible_users.clear();
  }
}

// This function takes in a TMI message and outputs the data into a NlUser struct - which contains useful information about the user.
pub fn return_user_struct(msg: &tmi::Privmsg) -> NlUser {
    
    let user = NlUser {
      username: String::from(msg.sender().name()),
      is_subscribed: msg.badges().any(|badge| matches!(badge, Badge::Subscriber(_))),
    };
    user
  }

// Takes a max number (Generally eligible_users.len()) and returns a random number 0..max
pub fn random_number(max: i32) -> i32 {
    // We intended max to be the .len() of the vector/array of users eligible.
    let result = rand::thread_rng().gen_range(0..max);
    result
}

// Selects a random user from the eligible users list, returns that user.
pub fn select_winner(user_list: &mut Eligible) -> String {
    user_list.list_eligible();
    let winner = &user_list.eligible_users[random_number(user_list.eligible_users.len().try_into().unwrap()) as usize];
    info!("\nWinner was: {} - {:?}\n", &winner.username, user_list);
    winner.username.clone()
}

// This logs to output.txt
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