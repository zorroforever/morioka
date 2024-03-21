use redis::{Commands, RedisResult};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub async fn make_token(
) -> String {
    let mut rng = thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .filter(|c| c.is_ascii_alphabetic() && c.is_uppercase())
        .take(20)
        .collect();

    let formatted_string = format!(
        "{}-{}-{}-{}-{}-{}",
        &random_string[0..5],
        &random_string[5..10],
        &random_string[10..14],
        &random_string[14..18],
        &random_string[18..22],
        &random_string[22..26],
    );

    formatted_string
}
#[tokio::test]
async fn test_make(){
    println!("{}", make_token().await);
}