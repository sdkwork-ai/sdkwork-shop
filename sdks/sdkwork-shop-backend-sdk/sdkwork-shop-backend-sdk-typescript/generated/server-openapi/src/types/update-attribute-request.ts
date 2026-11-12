/** The mutable surface of an attribute: display name, unit, ordering, and lifecycle status. The type facts (valueType, inputHint, isMultiValue) are create-time only and deliberately absent here. */
export interface UpdateAttributeRequest {
  /** Replacement display name. Bound by ck_commerce_product_attribute_name_length (1..100). */
  name?: string;
  /** Three-state. Absent leaves the stored unit alone; an explicit null removes it; a string replaces it. */
  unitSymbol?: string | null;
  /** Sibling ordering key, ascending. */
  sortOrder?: string;
  /** Shared lifecycle vocabulary. The authority for this set is the column CHECK constraint named in the field description. */
  status?: 'active' | 'inactive';
  /** The locale the text fields are written in. Omitted — or the default language — writes the base row; any other locale writes a translation row and leaves the base row alone. */
  locale?: string;
}
