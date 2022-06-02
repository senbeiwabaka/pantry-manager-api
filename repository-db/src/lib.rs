use std::time::Duration;

use rocket::async_trait;
use sea_orm::ConnectOptions;

use sea_orm_rocket::{rocket::figment::Figment, Config, Database};

#[derive(Database, Debug)]
#[database("pantry_manager")]
pub struct Db(SeaOrmPool);

#[derive(Clone, Debug)]
pub struct SeaOrmPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Connection = sea_orm::DatabaseConnection;

    type Error = sea_orm::DbErr;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>().unwrap();
        let mut options: ConnectOptions = config.url.into();

        options
            .max_connections(config.max_connections as u32)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout));

        let conn = sea_orm::Database::connect(options).await?;

        Ok(SeaOrmPool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}
