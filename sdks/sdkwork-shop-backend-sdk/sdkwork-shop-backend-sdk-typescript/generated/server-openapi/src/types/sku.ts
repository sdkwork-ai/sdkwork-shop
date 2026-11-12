export interface Sku {
  /** Primary key. */
  id: string;
  /** The tenant the SKU belongs to. Always the caller's own scope. */
  tenantId: string;
  /** The organization the SKU belongs to. Always the caller's own scope. */
  organizationId: string;
  /** Owning product id. */
  productId: string;
  /** Stable business key of the SKU, unique per tenant while the row is live. */
  skuNo: string;
  /** Deterministic signature of the attribute values that define this variant. Bounded by ck_commerce_product_sku_variant_signature. */
  variantSignature: string;
  /** Default-locale SKU name. */
  name?: string | null;
  /** Default-locale SKU title. */
  title?: string | null;
  /** ISO 4217 code every amount on this row is stated in. */
  currencyCode: string;
  /** The currency's display symbol, resolved from the currency table at read time. Presentation metadata a console renders next to the minor-unit amount, never a conversion factor. */
  currencySymbol: string;
  /** Minor-unit exponent of `currencyCode`, snapshotted from commerce_currency at write time. This is a unit exponent, not an amount: naming it `priceScale` reads as money to the API_SPEC section 13.2 validator, while the storage column keeps its own `price_scale` name. `price_scale BETWEEN 0 AND 8` per ck_commerce_product_sku_price_scale. */
  minorUnitExponent: string;
  /** Price actually charged, in the currency minor unit. Never negative (ck_commerce_product_sku_sale_price) and never above listPriceMinor (ck_commerce_product_sku_sale_not_above_list). */
  salePriceMinor: string;
  /** Reference price for the strike-through figure, in the same minor unit. Null means "not declared". */
  listPriceMinor?: string | null;
  /** The cost of acquiring one unit, in the same minor unit. It is what a margin needs and is never shown to a buyer. ck_commerce_product_sku_cost_price pins >= 0. */
  costPriceMinor?: string | null;
  /** The SKU-level barcode (GTIN/EAN), unique per tenant while the row is live (uk_commerce_product_sku_tenant_barcode). */
  barcode?: string | null;
  /** Ship weight in grams, the figure logistics rates a shipment from. ck_commerce_product_sku_weight pins >= 0. */
  weightGram?: string | null;
  /** The quantity a single order must buy at least. Defaults to 1; ck_commerce_product_sku_min_order_quantity pins >= 1. */
  minOrderQuantity: string;
  /** The quantity a single order may buy at most, when capped. ck_commerce_product_sku_max_order_quantity refuses a cap below the minimum. */
  maxOrderQuantity?: string | null;
  /** The step the orderable quantity moves in (a case, a dozen). Defaults to 1. */
  stepQuantity: string;
  /** The unit the buyer names (a box, a can), when declared. */
  packageUnit?: string | null;
  /** How many sellable units one package holds, when declared. */
  unitsPerPackage?: string | null;
  /** How one unit reaches the buyer. ck_commerce_product_sku_fulfillment_type is the authority for this set. */
  fulfillmentType: 'physical' | 'digital' | 'service' | 'membership_activation' | 'points_topup';
  /** Whether stock is tracked. ck_commerce_product_sku_inventory_tracking is the authority for this set. The stock itself is owned by sdkwork-inventory. */
  inventoryTracking: 'none' | 'quantity';
  /** What happens when the tracked stock runs out. `backorder` is only legal on a quantity-tracked SKU: ck_commerce_product_sku_policy_requires_tracking is the authority for the pairing. */
  inventoryPolicy: 'deny' | 'backorder';
  /** Lifecycle status. ck_commerce_product_sku_status is the authority for this set. */
  status: 'draft' | 'active' | 'inactive' | 'archived';
  /** Whether the SKU may be sold. `active` requires an active status (ck_commerce_product_sku_sales_requires_active). */
  salesStatus: 'active' | 'inactive';
  /** First publish time. Null until the SKU leaves draft (ck_commerce_product_sku_published_at). */
  publishedAt?: string | null;
  /** Row creation time. */
  createdAt: string;
  /** Last write time. */
  updatedAt: string;
  /** The SKU positions on its product category sales axes. Always present, empty when the category declares no sales axis: it is a list rather than a nullable field so "no axes" and "axes not loaded" cannot look the same. `variantSignature` carries the same information as one string; these rows are what a variant matrix is rendered from. */
  attributeValues: { attributeId: string; attributeValueId: string; attributeNo: string; valueCode: string; displayValue: string; }[];
  /** Row version, advanced by every write to this row and by any write that rewrites the row's own storage identity (a category move also advances the moved subtree). It is the value an update or delete must send back as `If-Match`, and the same number the response's `ETag` carries, so a client that has read a resource can always name the version it read. `API_SPEC` section 17 keeps the version source, the required header, and the two result codes together: a missing `If-Match` answers `42801`, a stale one answers `41201`. */
  version: string;
}
