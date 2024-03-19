use redis::{Commands, RedisResult};

fn set_token_with_expiry(
    con: &mut redis::Connection,
    key: &str,
    value: &str,
    expiry: usize
) -> RedisResult<()> {
    con.set_ex(key, value, expiry)?;
    Ok(())
}

fn check_token_validity(
    con: &mut redis::Connection,
    key: &str
) -> RedisResult<bool> {
    con.exists(key)
}

#[tokio::test]
async fn test_redis() -> RedisResult<()> {

    let token_key = "token_key";
    let token_value = "your_token_value";
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    set_token_with_expiry(&con, token_key, token_value, 86400)?;

    if check_token_validity(&con, token_key)? {
        println!("Token valid.");
    } else {
        println!("Token out of date.");
    }
    Ok(())
}
