export interface AttributeValue {
  /** Primary key. */
  id: string;
  /** The `enum` attribute this value belongs to. */
  attributeId: string;
  /** Stable business key of the value, unique per attribute within a tenant while the row is live. */
  valueCode: string;
  /** Buyer-facing display text. Bounded by ck_commerce_product_attribute_value_display_length (1..200). */
  displayValue: string;
  /** Swatch colour, `#RRGGBB`, when the value declares one. Bound by ck_commerce_product_attribute_value_color_hex. */
  colorHex?: string | null;
  /** The Drive resource id of the value's picture, when the value declares one. */
  mediaResourceId?: string | null;
  /** Sibling ordering key, ascending. */
  sortOrder: string;
  /** Shared lifecycle vocabulary. The authority for this set is the column CHECK constraint named in the field description. */
  status: 'active' | 'inactive';
  /** Row creation time. */
  createdAt: string;
  /** Last write time. */
  updatedAt: string;
  /** Row version, advanced by every write to this row and by any write that retires it. Echo it back as `If-Match`. */
  version: string;
}
