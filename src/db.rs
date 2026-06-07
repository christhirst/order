use deadpool_oracle::PoolBuilder;
use oracle_rs::Config;

use crate::settings::DatabaseSettings;

/// Builds an Oracle connection pool from the given database settings.
pub async fn create_pool(
    db: &DatabaseSettings,
) -> Result<deadpool_oracle::Pool, Box<dyn std::error::Error>> {
    let config = Config::new(&db.host, db.port, &db.service, &db.user, &db.password);

    let pool = PoolBuilder::new(config)
        .max_size(db.pool_size)
        .build()?;

    Ok(pool)
}
