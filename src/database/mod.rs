pub mod error;
pub mod indexes;

use mongodb::{options::ClientOptions, Client, Database};
use std::env;
use std::time::Duration;

pub use error::DatabaseError;
pub use indexes::create_all_indexes;

pub struct DatabaseConfig {
    pub database: Database,
    pub client: Client,
}

impl DatabaseConfig {
    pub async fn new() -> Result<Self, DatabaseError> {
        dotenv::dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

        let database_name =
            env::var("DATABASE_NAME").unwrap_or_else(|_| "rust-course-blog".to_string());

        let mut client_options = ClientOptions::parse(&database_url).await?;

        client_options.max_pool_size = Some(10);
        client_options.min_pool_size = Some(1);
        client_options.max_idle_time = Some(Duration::from_secs(300)); // 5 minutes
        client_options.connect_timeout = Some(Duration::from_secs(10));
        client_options.server_selection_timeout = Some(Duration::from_secs(30));

        let client = Client::with_options(client_options)?;

        client
            .database(&database_name)
            .run_command(mongodb::bson::doc! {"ping": 1}, None)
            .await?;

        let database = client.database(&database_name);

        Ok(DatabaseConfig { database, client })
    }

    pub fn get_database(&self) -> &Database {
        &self.database
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub async fn initialize_indexes(&self) -> Result<(), DatabaseError> {
        create_all_indexes(&self.database).await
    }
}
