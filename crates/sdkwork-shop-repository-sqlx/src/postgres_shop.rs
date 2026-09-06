use sdkwork_contract_service::CommerceServiceError;
use sdkwork_shop_service::{
    ShopDetailQuery, ShopListQuery, ShopPage, ShopScopeQuery, ShopSummaryView,
};
use sqlx::{Column, PgPool, Row};

#[derive(Debug, Clone)]
pub struct PostgresCommerceShopStore {
    pool: PgPool,
}

impl PostgresCommerceShopStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn list_shops(
        &self,
        query: ShopListQuery,
    ) -> Result<ShopPage<ShopSummaryView>, CommerceServiceError> {
        let offset = ((query.page - 1) * query.page_size) as i64;
        let limit = query.page_size as i64;
        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM commerce_shop
            WHERE tenant_id = CAST($1 AS TEXT)
              AND ($2 IS NULL OR organization_id = CAST($3 AS TEXT))
              AND deleted_at IS NULL
           "#,
        )
        .bind(&query.tenant_id)
        .bind(query.organization_id.as_deref())
        .bind(query.organization_id.as_deref())
        .fetch_one(self.pool())
        .await
        .map_err(|error| store_error("failed to count shops", error))?;

        let rows = sqlx::query(
            r#"
            SELECT id, tenant_id, organization_id, shop_no, shop_name, shop_type, business_model,
                   storefront_status, operation_status, review_status, data_scope,
                   logo_media_resource_id, cover_media_resource_id, default_currency_code,
                   default_locale, timezone, version, created_at, updated_at
            FROM commerce_shop
            WHERE tenant_id = CAST($1 AS TEXT)
              AND ($2 IS NULL OR organization_id = CAST($3 AS TEXT))
              AND deleted_at IS NULL
            ORDER BY created_at DESC, id DESC
            LIMIT $4 OFFSET $5
           "#,
        )
        .bind(&query.tenant_id)
        .bind(query.organization_id.as_deref())
        .bind(query.organization_id.as_deref())
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool())
        .await
        .map_err(|error| store_error("failed to list shops", error))?;

        Ok(ShopPage {
            items: rows.iter().map(map_shop_summary).collect(),
            page: query.page,
            page_size: query.page_size,
            total: total as u64,
        })
    }

    pub async fn retrieve_shop(
        &self,
        query: ShopDetailQuery,
    ) -> Result<Option<ShopSummaryView>, CommerceServiceError> {
        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, organization_id, shop_no, shop_name, shop_type, business_model,
                   storefront_status, operation_status, review_status, data_scope,
                   logo_media_resource_id, cover_media_resource_id, default_currency_code,
                   default_locale, timezone, version, created_at, updated_at
            FROM commerce_shop
            WHERE tenant_id = CAST($1 AS TEXT)
              AND id = CAST($2 AS TEXT)
              AND ($3 IS NULL OR organization_id = CAST($4 AS TEXT))
              AND deleted_at IS NULL
            LIMIT 1
           "#,
        )
        .bind(&query.tenant_id)
        .bind(&query.shop_id)
        .bind(query.organization_id.as_deref())
        .bind(query.organization_id.as_deref())
        .fetch_optional(self.pool())
        .await
        .map_err(|error| store_error("failed to retrieve shop", error))?;

        Ok(row.map(|row| map_shop_summary(&row)))
    }

    pub async fn retrieve_current_shop(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Option<ShopSummaryView>, CommerceServiceError> {
        let base_sql = r#"
            SELECT id, tenant_id, organization_id, shop_no, shop_name, shop_type, business_model,
                   storefront_status, operation_status, review_status, data_scope,
                   logo_media_resource_id, cover_media_resource_id, default_currency_code,
                   default_locale, timezone, version, created_at, updated_at
            FROM commerce_shop
            WHERE tenant_id = CAST($1 AS TEXT)
              AND organization_id = CAST($2 AS TEXT)
              AND deleted_at IS NULL
        "#;
        let sql = format!(
            "{base_sql} {}",
            crate::shop_current_selection::CURRENT_SHOP_ORDER_BY
        );
        let row = sqlx::query(sqlx::AssertSqlSafe(sql.as_str()))
            .bind(&scope.tenant_id)
            .bind(scope.organization_id.as_deref().unwrap_or(""))
            .fetch_optional(self.pool())
            .await
            .map_err(|error| store_error("failed to retrieve current shop", error))?;

        Ok(row.map(|row| map_shop_summary(&row)))
    }

    pub async fn resolve_current_shop_id(
        &self,
        scope: &ShopScopeQuery,
    ) -> Result<String, CommerceServiceError> {
        let organization_id = scope.organization_id.as_deref().unwrap_or("0");
        let base_sql = r#"
            SELECT id
            FROM commerce_shop
            WHERE tenant_id = CAST($1 AS TEXT)
              AND organization_id = CAST($2 AS TEXT)
              AND deleted_at IS NULL
        "#;
        let sql = format!(
            "{base_sql} {}",
            crate::shop_current_selection::CURRENT_SHOP_ORDER_BY
        );
        let shop_id: Option<String> = sqlx::query_scalar(sqlx::AssertSqlSafe(sql.as_str()))
            .bind(&scope.tenant_id)
            .bind(organization_id)
            .fetch_optional(self.pool())
            .await
            .map_err(|error| store_error("failed to resolve current shop", error))?;

        shop_id.ok_or_else(|| CommerceServiceError::not_found("current shop was not found"))
    }

    pub async fn list_dashboard_snapshots(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_metric_snapshot", &scope.tenant_id, &shop_id)
            .await
    }

    pub async fn list_settlements(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<ShopPage<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        let page = 1u32;
        let page_size = 20u32;
        let limit = page_size as i64;
        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM commerce_shop_metric_snapshot
            WHERE tenant_id = CAST($1 AS TEXT)
              AND shop_id = CAST($2 AS TEXT)
            "#,
        )
        .bind(&scope.tenant_id)
        .bind(&shop_id)
        .fetch_one(self.pool())
        .await
        .map_err(|error| store_error("failed to count shop settlements", error))?;
        let rows = sqlx::query(
            r#"
            SELECT *
            FROM commerce_shop_metric_snapshot
            WHERE tenant_id = CAST($1 AS TEXT)
              AND shop_id = CAST($2 AS TEXT)
            ORDER BY snapshot_date DESC, id DESC
            LIMIT $3
            "#,
        )
        .bind(&scope.tenant_id)
        .bind(&shop_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|error| store_error("failed to list shop settlements", error))?;
        Ok(ShopPage {
            items: rows.iter().map(map_row_json).collect(),
            page,
            page_size,
            total: total.max(0) as u64,
        })
    }

    pub async fn list_shop_orders(
        &self,
        scope: ShopScopeQuery,
        page: u32,
        page_size: u32,
    ) -> Result<ShopPage<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        let offset = ((page.max(1) - 1) * page_size.clamp(1, 200)) as i64;
        let limit = page_size.clamp(1, 200) as i64;
        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(DISTINCT o.id)
            FROM commerce_order o
            INNER JOIN commerce_order_item i
                ON i.tenant_id = o.tenant_id
               AND i.order_id = o.id
            WHERE o.tenant_id = CAST($1 AS TEXT)
              AND i.shop_id = CAST($2 AS TEXT)
           "#,
        )
        .bind(&scope.tenant_id)
        .bind(&shop_id)
        .fetch_one(self.pool())
        .await
        .map_err(|error| store_error("failed to count shop orders", error))?;

        let rows = sqlx::query(
            r#"
            SELECT DISTINCT o.id, o.tenant_id, o.organization_id, o.owner_user_id, o.order_no,
                   o.status, o.payment_status, o.fulfillment_status, o.refund_status, o.subject,
                   o.currency_code, o.request_no, o.idempotency_key, o.created_at, o.paid_at,
                   o.cancelled_at, o.expired_at, o.updated_at
            FROM commerce_order o
            INNER JOIN commerce_order_item i
                ON i.tenant_id = o.tenant_id
               AND i.order_id = o.id
            WHERE o.tenant_id = CAST($1 AS TEXT)
              AND i.shop_id = CAST($2 AS TEXT)
            ORDER BY o.created_at DESC, o.id DESC
            LIMIT $3 OFFSET $4
           "#,
        )
        .bind(&scope.tenant_id)
        .bind(&shop_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool())
        .await
        .map_err(|error| store_error("failed to list shop orders", error))?;

        Ok(ShopPage {
            items: rows.iter().map(map_row_json).collect(),
            page: page.max(1),
            page_size: page_size.clamp(1, 200),
            total: total as u64,
        })
    }

    pub async fn retrieve_shop_order(
        &self,
        scope: ShopScopeQuery,
        order_id: &str,
    ) -> Result<Option<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        let row = sqlx::query(
            r#"
            SELECT o.id, o.tenant_id, o.organization_id, o.owner_user_id, o.order_no,
                   o.status, o.payment_status, o.fulfillment_status, o.refund_status, o.subject,
                   o.currency_code, o.request_no, o.idempotency_key, o.created_at, o.paid_at,
                   o.cancelled_at, o.expired_at, o.updated_at
            FROM commerce_order o
            INNER JOIN commerce_order_item i
                ON i.tenant_id = o.tenant_id
               AND i.order_id = o.id
            WHERE o.tenant_id = CAST($1 AS TEXT)
              AND i.shop_id = CAST($2 AS TEXT)
              AND o.id = CAST($3 AS TEXT)
            LIMIT 1
           "#,
        )
        .bind(&scope.tenant_id)
        .bind(&shop_id)
        .bind(order_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|error| store_error("failed to retrieve shop order", error))?;

        Ok(row.map(|row| map_row_json(&row)))
    }

    pub async fn create_shop_fulfillment(
        &self,
        scope: ShopScopeQuery,
        order_id: &str,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        let organization_id = scope.organization_id.as_deref().unwrap_or("");
        let now = chrono_like_now();
        let fulfillment_id = payload
            .get("fulfillmentId")
            .or_else(|| payload.get("fulfillment_id"))
            .and_then(|value| value.as_str())
            .unwrap_or("fulfillment-generated");
        let fulfillment_no = payload
            .get("fulfillmentNo")
            .or_else(|| payload.get("fulfillment_no"))
            .and_then(|value| value.as_str())
            .unwrap_or("FF-GEN");
        let fulfillment_type = payload
            .get("fulfillmentType")
            .or_else(|| payload.get("fulfillment_type"))
            .and_then(|value| value.as_str())
            .unwrap_or("physical");
        sqlx::query(
            r#"
            INSERT INTO commerce_fulfillment_order
                (id, tenant_id, organization_id, fulfillment_no, order_id, shop_id,
                 fulfillment_type, status, request_no, idempotency_key, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, 'pending', $8, $9, $10, $11)
           "#,
        )
        .bind(fulfillment_id)
        .bind(&scope.tenant_id)
        .bind(organization_id)
        .bind(fulfillment_no)
        .bind(order_id)
        .bind(&shop_id)
        .bind(fulfillment_type)
        .bind(fulfillment_no)
        .bind(format!("idem-{fulfillment_id}"))
        .bind(&now)
        .bind(&now)
        .execute(self.pool())
        .await
        .map_err(|error| store_error("failed to create shop fulfillment", error))?;

        self.find_table_row_by_column(
            "commerce_fulfillment_order",
            &scope.tenant_id,
            fulfillment_id,
            "id",
        )
        .await
        .and_then(|value| {
            value.ok_or_else(|| {
                CommerceServiceError::storage("fulfillment row missing after insert")
            })
        })
    }

    pub async fn list_category_bindings(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_category_binding", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn list_brand_authorizations(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows(
            "commerce_shop_brand_authorization",
            &scope.tenant_id,
            &shop_id,
        )
        .await
    }
    pub async fn list_qualifications(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_qualification", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn list_customer_services(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_customer_service", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn list_return_addresses(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_return_address", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn list_shipping_templates(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows(
            "commerce_shop_shipping_template",
            &scope.tenant_id,
            &shop_id,
        )
        .await
    }
    pub async fn list_applications(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_application", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn list_verifications(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_verification", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn list_status_events(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_status_event", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn list_channels(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_channel", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn list_service_areas(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_service_area", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn list_policies(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_policy", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn list_risk_signals(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.list_shop_table_rows("commerce_shop_risk_signal", &scope.tenant_id, &shop_id)
            .await
    }

    pub async fn find_fulfillment_profile(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Option<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.find_shop_table_row(
            "commerce_shop_fulfillment_profile",
            &scope.tenant_id,
            &shop_id,
        )
        .await
    }
    pub async fn find_settlement_profile(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Option<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.find_shop_table_row(
            "commerce_shop_settlement_profile",
            &scope.tenant_id,
            &shop_id,
        )
        .await
    }
    pub async fn find_business_hours(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Option<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.find_shop_table_row("commerce_shop_business_hour", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn find_readiness(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Option<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.find_shop_table_row("commerce_shop_readiness", &scope.tenant_id, &shop_id)
            .await
    }
    pub async fn find_deposit_account(
        &self,
        scope: ShopScopeQuery,
    ) -> Result<Option<serde_json::Value>, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.find_shop_table_row("commerce_shop_deposit_account", &scope.tenant_id, &shop_id)
            .await
    }

    pub async fn upsert_category_bindings(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload("commerce_shop_category_binding", &scope, &shop_id, payload)
            .await
    }
    pub async fn upsert_brand_authorizations(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload(
            "commerce_shop_brand_authorization",
            &scope,
            &shop_id,
            payload,
        )
        .await
    }
    pub async fn upsert_qualifications(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload("commerce_shop_qualification", &scope, &shop_id, payload)
            .await
    }
    pub async fn upsert_customer_services(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload("commerce_shop_customer_service", &scope, &shop_id, payload)
            .await
    }
    pub async fn upsert_return_addresses(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload("commerce_shop_return_address", &scope, &shop_id, payload)
            .await
    }
    pub async fn upsert_shipping_templates(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload("commerce_shop_shipping_template", &scope, &shop_id, payload)
            .await
    }

    pub async fn upsert_applications(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload("commerce_shop_application", &scope, &shop_id, payload)
            .await
    }

    pub async fn upsert_channels(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload("commerce_shop_channel", &scope, &shop_id, payload)
            .await
    }

    pub async fn upsert_service_areas(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload("commerce_shop_service_area", &scope, &shop_id, payload)
            .await
    }

    pub async fn upsert_policies(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload("commerce_shop_policy", &scope, &shop_id, payload)
            .await
    }

    pub async fn upsert_fulfillment_profile(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload(
            "commerce_shop_fulfillment_profile",
            &scope,
            &shop_id,
            payload,
        )
        .await
    }

    pub async fn upsert_settlement_profile(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload(
            "commerce_shop_settlement_profile",
            &scope,
            &shop_id,
            payload,
        )
        .await
    }

    pub async fn upsert_business_hours(
        &self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        let shop_id = self.resolve_current_shop_id(&scope).await?;
        self.upsert_shop_payload("commerce_shop_business_hour", &scope, &shop_id, payload)
            .await
    }

    async fn list_shop_table_rows(
        &self,
        table: &str,
        tenant_id: &str,
        shop_id: &str,
    ) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
        let sql = format!(
            "SELECT * FROM {table} WHERE tenant_id = CAST(? AS TEXT) AND shop_id = CAST(? AS TEXT) ORDER BY created_at DESC, id DESC"
        );
        let rows = sqlx::query(sqlx::AssertSqlSafe(sql.as_str()))
            .bind(tenant_id)
            .bind(shop_id)
            .fetch_all(self.pool())
            .await
            .map_err(|error| {
                store_error(
                    &format!("failed to list {table} rows", table = table),
                    error,
                )
            })?;
        Ok(rows.iter().map(map_row_json).collect())
    }

    async fn find_shop_table_row(
        &self,
        table: &str,
        tenant_id: &str,
        shop_id: &str,
    ) -> Result<Option<serde_json::Value>, CommerceServiceError> {
        let sql = format!(
            "SELECT * FROM {table} WHERE tenant_id = CAST(? AS TEXT) AND shop_id = CAST(? AS TEXT) LIMIT 1"
        );
        let row = sqlx::query(sqlx::AssertSqlSafe(sql.as_str()))
            .bind(tenant_id)
            .bind(shop_id)
            .fetch_optional(self.pool())
            .await
            .map_err(|error| {
                store_error(&format!("failed to find {table} row", table = table), error)
            })?;
        Ok(row.map(|row| map_row_json(&row)))
    }

    async fn find_table_row_by_column(
        &self,
        table: &str,
        tenant_id: &str,
        key_value: &str,
        key_column: &str,
    ) -> Result<Option<serde_json::Value>, CommerceServiceError> {
        let sql = format!(
            "SELECT * FROM {table} WHERE tenant_id = CAST(? AS TEXT) AND {key_column} = CAST(? AS TEXT) LIMIT 1",
            table = table,
            key_column = key_column,
        );
        let row = sqlx::query(sqlx::AssertSqlSafe(sql.as_str()))
            .bind(tenant_id)
            .bind(key_value)
            .fetch_optional(self.pool())
            .await
            .map_err(|error| {
                store_error(&format!("failed to find {table} row", table = table), error)
            })?;
        Ok(row.map(|row| map_row_json(&row)))
    }

    async fn upsert_shop_payload(
        &self,
        table: &str,
        scope: &ShopScopeQuery,
        shop_id: &str,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, CommerceServiceError> {
        crate::shop_subresource_upsert::upsert_shop_table_row(
            &crate::shop_subresource_upsert::ShopWriteDb::postgres(self.pool().clone()),
            table,
            &scope.tenant_id,
            scope.organization_id.as_deref(),
            shop_id,
            payload
                .get("id")
                .and_then(|value| value.as_str())
                .map(str::to_owned),
            payload,
        )
        .await
    }
}

fn map_shop_summary(row: &sqlx::postgres::PgRow) -> ShopSummaryView {
    ShopSummaryView {
        shop_id: string_cell(row, "id"),
        tenant_id: string_cell(row, "tenant_id"),
        organization_id: string_cell(row, "organization_id"),
        shop_no: string_cell(row, "shop_no"),
        shop_name: string_cell(row, "shop_name"),
        shop_type: string_cell(row, "shop_type"),
        business_model: string_cell(row, "business_model"),
        storefront_status: string_cell(row, "storefront_status"),
        operation_status: string_cell(row, "operation_status"),
        review_status: string_cell(row, "review_status"),
        data_scope: string_cell(row, "data_scope"),
        logo_media_resource_id: optional_string_cell(row, "logo_media_resource_id"),
        cover_media_resource_id: optional_string_cell(row, "cover_media_resource_id"),
        default_currency_code: string_cell(row, "default_currency_code"),
        default_locale: optional_string_cell(row, "default_locale"),
        timezone: optional_string_cell(row, "timezone"),
        version: row.try_get::<i64, _>("version").unwrap_or(0),
        created_at: string_cell(row, "created_at"),
        updated_at: string_cell(row, "updated_at"),
    }
}

fn map_row_json(row: &sqlx::postgres::PgRow) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for column in row.columns() {
        let name = column.name();
        let key = snake_to_camel(name);
        let value = if name == "id" {
            serde_json::Value::String(string_cell(row, name))
        } else if name.ends_with("_id") && name != "id" {
            optional_string_cell(row, name)
                .map(serde_json::Value::String)
                .unwrap_or(serde_json::Value::Null)
        } else if name.ends_with("_json") {
            optional_string_cell(row, name)
                .and_then(|text| serde_json::from_str(&text).ok())
                .unwrap_or(serde_json::Value::Object(serde_json::Map::new()))
        } else {
            optional_string_cell(row, name)
                .map(serde_json::Value::String)
                .unwrap_or(serde_json::Value::Null)
        };
        map.insert(key, value);
    }
    serde_json::Value::Object(map)
}

fn snake_to_camel(name: &str) -> String {
    let parts: Vec<&str> = name.split('_').collect();
    if parts.is_empty() {
        return String::new();
    }
    parts[0].to_owned()
        + &parts[1..]
            .iter()
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>()
}

fn chrono_like_now() -> String {
    "2026-06-17 00:00:00".to_owned()
}

fn store_error(message: &str, error: impl std::fmt::Display) -> CommerceServiceError {
    CommerceServiceError::storage(format!("{message}: {error}"))
}

fn optional_string_cell(row: &sqlx::postgres::PgRow, column: &str) -> Option<String> {
    row.try_get::<Option<String>, _>(column).ok().flatten()
}

fn string_cell(row: &sqlx::postgres::PgRow, column: &str) -> String {
    optional_string_cell(row, column).unwrap_or_default()
}
