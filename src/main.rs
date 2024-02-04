fn main() {
    let mut i = 1;
    while random_number(500000000) != 0 {
        i += 1;
    }
    println!("It took {} attempts to hit 0", i);
}



use rand::Rng;



fn random_number(max: i32) -> i32 {
    // We intended max to be the .len() of the vector/array of users eligible.
    let mut result = rand::thread_rng().gen_range(0..max);
    result
}