/** The identity pair (attributeId, valueCode) is deliberately absent: moving a value between attributes or renaming its key is a delete plus a create, not an edit. */
export interface UpdateAttributeValueRequest {
  /** Replacement display text. Bound by ck_commerce_product_attribute_value_display_length (1..200). */
  displayValue?: string;
  /** Three-state. Absent leaves the stored colour alone; an explicit null removes the swatch; a string replaces it. */
  colorHex?: string | null;
  /** Three-state. Absent leaves the stored picture alone; an explicit null removes it; an id replaces it. */
  mediaResourceId?: string | null;
  /** Sibling ordering key, ascending. */
  sortOrder?: string;
  /** Shared lifecycle vocabulary. The authority for this set is the column CHECK constraint named in the field description. */
  status?: 'active' | 'inactive';
}
