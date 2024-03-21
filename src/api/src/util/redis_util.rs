use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands;

pub struct MoriokaRedis {
    pool: r2d2::Pool<RedisConnectionManager>,
}

impl MoriokaRedis {
    pub fn new(
        redis_url: &str
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let manager = RedisConnectionManager::new(redis_url)?;
        let pool = r2d2::Pool::builder().build(manager)?;
        Ok(MoriokaRedis { pool })
    }

    pub async fn set_token_with_expiry(
        &self,
        key: &str,
        value: &str,
        expiry: usize,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let _ = conn.set_ex(key, value, expiry)?;
        Ok(())
    }

    pub async fn check_token_validity(
        &self,
        key: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let result = conn.exists(key)?;
        Ok(result)
    }


    pub async fn get_val_by_key(
        &self,
        key: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let res: String = conn.get(key)?;
        Ok(res)
    }
}
