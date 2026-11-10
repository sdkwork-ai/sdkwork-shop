export interface UpdateSkuRequest {
  /** New default-locale name. */
  name?: string;
  /** New default-locale title. */
  title?: string;
  /** New charged price, in the currency's minor unit. Required whenever currencyCode changes, because the same integer means a different amount under another currency's exponent. */
  salePriceMinor?: string;
  /** New reference price in the same minor unit. Three-state on purpose: omitting the field leaves the stored price unchanged, an explicit `null` clears it, and a value replaces it. Clearing is stated rather than inferred, which is what lets a caller take a strike-through price away at all; `null` is also not `0`, which is a real price and would fail ck_commerce_product_sku_sale_not_above_list whenever the sale price is above it. */
  listPriceMinor?: string | null;
  /** ISO 4217 code of the currency every amount in this request is stated in. Bound by ck_commerce_currency_code_shape and by the foreign key to commerce_currency. */
  currencyCode?: string;
  /** New fulfillment type. */
  fulfillmentType?: 'physical' | 'digital' | 'service' | 'membership_activation' | 'points_topup';
  /** New inventory-tracking declaration. */
  inventoryTracking?: 'none' | 'quantity';
  /** Lifecycle status. ck_commerce_product_sku_status admits these four; a sellable SKU must also be 'active' (ck_commerce_product_sku_sales_requires_active), and retirement is a DELETE. */
  status?: 'draft' | 'active' | 'inactive' | 'archived';
  /** Positions this SKU occupies on the product category`s sales axes, as dictionary value ids. The attribute behind each value is derived server-side, so a request cannot name an attribute and a value that disagree, and the axes are written as a set. The resolved values must cover every active sales axis of the product category exactly once; a partial combination is refused. The variant signature is recomputed as `attributeNo=valueCode` terms ordered by `attributeNo` and joined with `;`, bounded by ck_commerce_product_sku_variant_signature. */
  attributeValueIds?: string[];
  /** Three-state, like listPriceMinor. */
  costPriceMinor?: string | null;
  /** Three-state. A restated barcode must be unique per tenant. */
  barcode?: string | null;
  /** Three-state. ck_commerce_product_sku_weight pins a restated value >= 0. */
  weightGram?: string | null;
  /** The repository checks the pairing against the *effective* values after the merge, because a command that only states the policy cannot see the stored tracking mode. */
  inventoryPolicy?: 'deny' | 'backorder';
}
