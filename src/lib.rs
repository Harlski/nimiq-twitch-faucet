use log::{debug, error, info, trace, warn};
use std::time::SystemTime;
use humantime::Rfc3339Timestamp;
use rand::Rng;

pub fn random_number(max: i32) -> i32 {
    // We intended max to be the .len() of the vector/array of users eligible.
    let mut result = rand::thread_rng().gen_range(0..max);
    result
}

pub fn select_winner(user_list: Vec<String>) -> String {
    let winner = &user_list[random_number(user_list.len().try_into().unwrap()) as usize];
    info!("Winner was: {} - {:?}", &winner, user_list);
    winner.to_string()
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