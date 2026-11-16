use async_trait::async_trait;
use chrono::Utc;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_shop_service::{CreateShopCommand, ShopProfile, ShopRepository, ShopSummary};
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// PostgreSQL adapter for the runtime shop port.
///
/// The physical `commerce_shop` table is the text-keyed commerce model the HTTP
/// repository reads (`shop_no`, `shop_name`, `storefront_status`, TEXT
/// timestamps); this adapter maps the runtime UUID domain types onto it so both
/// faces of the capability share one row instead of two competing schemas.
pub struct SqlxShopRepository {
    pool: PgPool,
}

impl SqlxShopRepository {
    pub fn new(pool: DatabasePool) -> Self {
        Self {
            pool: pool
                .as_postgres()
                .cloned()
                .expect("shop repository requires postgres DatabasePool"),
        }
    }
}

#[async_trait]
impl ShopRepository for SqlxShopRepository {
    async fn find_by_tenant_and_org(
        &self,
        tenant_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Option<ShopProfile>, String> {
        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, organization_id, shop_no, shop_name,
                   storefront_status, created_at, updated_at
            FROM commerce_shop
            WHERE tenant_id = $1 AND organization_id = $2
              AND deleted_at IS NULL
            ORDER BY created_at DESC, id DESC
            LIMIT 1
            "#,
        )
        .bind(tenant_id.to_string())
        .bind(organization_id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| format!("query shop profile failed: {error}"))?;

        Ok(row.map(map_shop_profile).transpose()?)
    }

    async fn create_shop(&self, command: CreateShopCommand) -> Result<ShopProfile, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let status = "draft".to_owned();
        let timestamp = now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

        sqlx::query(
            r#"
            INSERT INTO commerce_shop
                (id, tenant_id, organization_id, shop_no, shop_name, shop_type,
                 business_model, storefront_status, operation_status, review_status,
                 data_scope, version, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, 'physical',
                 'self_operated', $6, 'normal', 'pending',
                 'organization', 0, $7, $8)
            "#,
        )
        .bind(id.to_string())
        .bind(command.tenant_id.to_string())
        .bind(command.organization_id.to_string())
        .bind(&command.slug)
        .bind(&command.name)
        .bind(&status)
        .bind(&timestamp)
        .bind(&timestamp)
        .execute(&self.pool)
        .await
        .map_err(|error| format!("insert shop failed: {error}"))?;

        Ok(ShopProfile {
            id,
            tenant_id: command.tenant_id,
            organization_id: command.organization_id,
            name: command.name,
            slug: command.slug,
            status,
            created_at: now,
            updated_at: now,
        })
    }

    async fn list_for_admin(&self, tenant_id: Uuid) -> Result<Vec<ShopSummary>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, shop_name, storefront_status
            FROM commerce_shop
            WHERE tenant_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC, id DESC
            "#,
        )
        .bind(tenant_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|error| format!("list shops failed: {error}"))?;

        rows.into_iter()
            .map(|row| {
                Ok(ShopSummary {
                    id: parse_uuid_cell(&row, "id")?,
                    name: string_cell(&row, "shop_name"),
                    status: string_cell(&row, "storefront_status"),
                })
            })
            .collect()
    }
}

fn map_shop_profile(row: sqlx::postgres::PgRow) -> Result<ShopProfile, String> {
    Ok(ShopProfile {
        id: parse_uuid_cell(&row, "id")?,
        tenant_id: parse_uuid_cell(&row, "tenant_id")?,
        organization_id: parse_uuid_cell(&row, "organization_id")?,
        name: string_cell(&row, "shop_name"),
        slug: string_cell(&row, "shop_no"),
        status: string_cell(&row, "storefront_status"),
        created_at: parse_timestamp_cell(&row, "created_at")?,
        updated_at: parse_timestamp_cell(&row, "updated_at")?,
    })
}

fn parse_uuid_cell(row: &sqlx::postgres::PgRow, column: &str) -> Result<Uuid, String> {
    Uuid::parse_str(&string_cell(row, column))
        .map_err(|error| format!("commerce_shop.{column} is not a UUID: {error}"))
}

fn parse_timestamp_cell(
    row: &sqlx::postgres::PgRow,
    column: &str,
) -> Result<chrono::DateTime<Utc>, String> {
    chrono::DateTime::parse_from_rfc3339(&string_cell(row, column))
        .map(|value| value.with_timezone(&Utc))
        .map_err(|error| format!("commerce_shop.{column} is not RFC 3339: {error}"))
}

fn string_cell(row: &sqlx::postgres::PgRow, column: &str) -> String {
    row.try_get::<Option<String>, _>(column)
        .ok()
        .flatten()
        .unwrap_or_default()
}
