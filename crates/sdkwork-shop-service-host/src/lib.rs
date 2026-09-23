use std::sync::Arc;

use sdkwork_database_id::IdGenerator;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_shop_database_host::{bootstrap_shop_database_from_env, ShopDatabaseHost};
use sdkwork_shop_repository_sqlx::SqlxShopRepository;
use sdkwork_shop_service::ShopService;

pub mod identity;
pub mod runtime_env;

pub use identity::{shared_identity, ShopIdentity};

pub struct ShopServiceHost {
    database: ShopDatabaseHost,
    shop_service: ShopService<SqlxShopRepository>,
    identity: Arc<ShopIdentity>,
}

impl ShopServiceHost {
    pub async fn new() -> Self {
        Self::from_env()
            .await
            .expect("shop service host bootstrap failed")
    }

    pub async fn from_env() -> Result<Self, String> {
        let database = bootstrap_shop_database_from_env().await?;
        let repository = SqlxShopRepository::new(database.pool().clone());
        // One identity for the whole process: the app catalog routes build a merchandise catalog
        // store to mint merchandise ids, and those ids must come from this node id rather than from
        // a second, divergent generator.
        let identity = shared_identity(ShopIdentity::from_pool(database.pool()).await?);
        Ok(Self {
            shop_service: ShopService::new(repository),
            database,
            identity,
        })
    }

    /// Build the shop service host against a caller-provided database pool so
    /// the platform cloud gateway can share its process-wide PostgreSQL pool.
    pub async fn from_pool(pool: DatabasePool) -> Result<Self, String> {
        let database = sdkwork_shop_database_host::bootstrap_shop_database(pool).await?;
        let repository = SqlxShopRepository::new(database.pool().clone());
        let identity = shared_identity(ShopIdentity::from_pool(database.pool()).await?);
        Ok(Self {
            shop_service: ShopService::new(repository),
            database,
            identity,
        })
    }

    pub fn shop_service(&self) -> &ShopService<SqlxShopRepository> {
        &self.shop_service
    }

    pub fn database_pool(&self) -> &DatabasePool {
        self.database.pool()
    }

    pub fn database_module(&self) -> std::sync::Arc<sdkwork_database_spi::DefaultDatabaseModule> {
        self.database.module()
    }

    /// The process Snowflake identity.
    #[must_use]
    pub fn identity(&self) -> Arc<ShopIdentity> {
        Arc::clone(&self.identity)
    }

    /// The injectable Snowflake port repositories consume.
    ///
    /// One process, one node id: this returns the same generator to every caller, so a later
    /// repository cannot be wired to a divergent sequence.
    #[must_use]
    pub fn id_generator(&self) -> Arc<dyn IdGenerator> {
        self.identity.clone()
    }
}

pub fn default_seed_locale() -> &'static str {
    "zh-CN"
}

pub fn default_seed_profile() -> &'static str {
    "standard"
}
