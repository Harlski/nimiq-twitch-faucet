use redis::Commands;
use faucet::NlUser;

pub fn do_something(user: NlUser) -> redis::RedisResult<()> {
    let client = redis::Client::open("")?;
    let mut con = client.get_connection()?;
 
    
    let _ : () = redis::cmd("SET").arg("my_key").arg(42).query(&mut con)?;

    Ok(())
}