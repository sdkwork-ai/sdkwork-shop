export interface CreateAttributeValueRequest {
  /** The `enum` attribute whose dictionary this value extends. The repository re-checks, inside the transaction, that the attribute exists, belongs to the tenant, and is an `enum`. */
  attributeId: string;
  /** Stable business key of the value, unique per attribute within a tenant while the row is live (uk_commerce_product_attribute_value_tenant_attr_code). */
  valueCode: string;
  /** Buyer-facing display text. Bound by ck_commerce_product_attribute_value_display_length (1..200). */
  displayValue: string;
  /** Swatch colour, `#RRGGBB`. Bound by ck_commerce_product_attribute_value_color_hex. */
  colorHex?: string;
  /** The Drive resource id of the value's picture, as an int64 string. */
  mediaResourceId?: string;
  /** Sibling ordering key, ascending. Defaults to 0. */
  sortOrder?: string;
}
