export interface CreateSkuRequest {
  /** Product the SKU belongs to. */
  productId: string;
  /** Stable business key of the SKU, unique per tenant while the row is live (uk_commerce_product_sku_tenant_no). */
  skuNo: string;
  /** Default-locale SKU name. */
  name: string;
  /** Default-locale SKU title. */
  title: string;
  /** Price actually charged, in the currency's minor unit (fen for CNY, yen for JPY). API_SPEC section 13.2.1 fixes the wire unit at the minor unit, so 640 means 640 fen and no layer rescales it. */
  salePriceMinor: string;
  /** Optional reference price in the same minor unit, used for the strike-through figure. It is never below salePriceMinor (ck_commerce_product_sku_sale_not_above_list); omit it rather than sending 0 to mean "not declared". */
  listPriceMinor?: string;
  /** ISO 4217 code of the currency every amount in this request is stated in. Bound by ck_commerce_currency_code_shape and by the foreign key to commerce_currency. */
  currencyCode: string;
  /** How one unit of this SKU reaches the buyer. ck_commerce_product_sku_fulfillment_type is the authority for this set. */
  fulfillmentType: 'physical' | 'digital' | 'service' | 'membership_activation' | 'points_topup';
  /** Whether stock is tracked for this SKU. The stock itself is owned by sdkwork-inventory; this only declares the policy. ck_commerce_product_sku_inventory_tracking is the authority for this set. */
  inventoryTracking: 'none' | 'quantity';
  /** Positions this SKU occupies on the product category`s sales axes, as dictionary value ids. The attribute behind each value is derived server-side, so a request cannot name an attribute and a value that disagree, and the axes are written as a set. The resolved values must cover every active sales axis of the product category exactly once; a partial combination is refused. The variant signature is recomputed as `attributeNo=valueCode` terms ordered by `attributeNo` and joined with `;`, bounded by ck_commerce_product_sku_variant_signature. */
  attributeValueIds?: string[];
  /** The cost of acquiring one unit, in the same minor unit. It is what a margin needs and is never shown to a buyer. */
  costPriceMinor?: string;
  /** The SKU-level barcode (GTIN/EAN), unique per tenant while the row is live. */
  barcode?: string;
  /** Ship weight in grams, the figure logistics rates a shipment from. */
  weightGram?: string;
  /** What happens when the tracked stock runs out. `backorder` requires inventoryTracking `quantity` (ck_commerce_product_sku_policy_requires_tracking); the pairing is checked at the boundary with both fields named. */
  inventoryPolicy: 'deny' | 'backorder';
  /** The quantity a single order must buy at least. Defaults to 1. */
  minOrderQuantity?: string;
  /** The quantity a single order may buy at most, when capped. */
  maxOrderQuantity?: string;
  /** The step the orderable quantity moves in. Defaults to 1. */
  stepQuantity?: string;
  /** The unit the buyer names (a box, a can), when declared. */
  packageUnit?: string;
  /** How many sellable units one package holds, when declared. */
  unitsPerPackage?: string;
  /** The locale the text fields are written in. Omitted — or the default language — writes the base row; any other locale writes a translation row and leaves the base row alone. */
  locale?: string;
}
