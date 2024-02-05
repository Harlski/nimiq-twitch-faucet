use log::{info};

fn main() {
    faucet::setup_logger();

    let mut eligible_users: Vec<String> = vec![];

    // Just some logic to generate random sized arrays. 
    let mut eligible_amount = random_number(100);
    while eligible_amount > 0 {
        eligible_users.push("User_".to_string() + &eligible_amount.to_string());
        eligible_amount -= 1;
    }
    //

    println!("Eligible users: {:?}", eligible_users);
    if eligible_users.len() == 0 { println!("No eligible users."); info!("No eligible users."); }
    else { println!("Winner: {}", select_winner(eligible_users)); }

}



use rand::Rng;


fn select_winner(user_list: Vec<String>) -> String {
    let winner = &user_list[random_number(user_list.len().try_into().unwrap()) as usize];
    info!("Winner was: {} - {:?}", &winner, user_list);
    winner.to_string()
}

fn random_number(max: i32) -> i32 {
    // We intended max to be the .len() of the vector/array of users eligible.
    let mut result = rand::thread_rng().gen_range(0..max);
    result
}