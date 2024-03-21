use redis::{Commands, Connection, RedisResult};
use redis_pool::{RedisPool, SingleRedisPool};

pub struct MoriokaRedis {
    pool:redis_pool::SingleRedisPool,
}
impl MoriokaRedis{
    pub  fn new(
        redis_url: &str
    )-> Self {
        let client = redis::Client::open(redis_url).expect("redis error on open");
        let pool = RedisPool::from(client);
        crate::util::redis_util::MoriokaRedis{pool}
    }
    pub async fn set_token_with_expiry(
        &self,
        key: &str,
        value: &str,
        expiry: usize
    ) -> RedisResult<()> {
        let mut con = self.pool.aquire().await.unwrap();
        let _ = con.set_ex(key, value, expiry)?;
        Ok(())
    }

    pub async fn check_token_validity(
        &self,
        key: &str
    ) -> RedisResult<bool> {
        let mut con = self.pool.aquire().await.unwrap().detach();

        redis::pipe()
            .exists(key)
            .execute(&mut con)
            .await.unwrap()
    }


    pub async fn get_val_by_key(
        &self,
        key: &str
    )-> RedisResult<String> {
        let mut con = self.pool.aquire().await.unwrap();
        con.get(key)
    }
}
