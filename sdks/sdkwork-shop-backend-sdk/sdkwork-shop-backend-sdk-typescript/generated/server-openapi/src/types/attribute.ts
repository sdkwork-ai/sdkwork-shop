export interface Attribute {
  /** Primary key. */
  id: string;
  /** Stable business key of the attribute, unique per tenant while the row is live. */
  attributeNo: string;
  /** Default-locale name. Bounded by ck_commerce_product_attribute_name_length. */
  name: string;
  /** Declared value type. ck_commerce_product_attribute_value_type is the authority for this set. */
  valueType: 'enum' | 'text' | 'number' | 'bool' | 'date';
  /** The control a console renders for this attribute. `ck_commerce_product_attribute_input_hint` is the authority for this set. */
  inputHint: 'select' | 'multi_select' | 'text' | 'number' | 'toggle' | 'date';
  /** Whether the attribute accepts more than one dictionary value per product. Only an `enum` attribute can carry this (ck_commerce_product_attribute_multi_value_needs_enum). */
  isMultiValue: boolean;
  /** The unit the attribute's values are denominated in (`cm`, `kg`), when one is declared. */
  unitSymbol?: string | null;
  /** Shared lifecycle vocabulary. The authority for this set is the column CHECK constraint named in the field description. */
  status: 'active' | 'inactive';
  /** Sibling ordering key, ascending. */
  sortOrder: string;
  /** Row creation time. */
  createdAt: string;
  /** Last write time. */
  updatedAt: string;
  /** Row version, advanced by every write to this row and by any write that rewrites the row's own storage identity (a category move also advances the moved subtree). It is the value an update or delete must send back as `If-Match`, and the same number the response's `ETag` carries, so a client that has read a resource can always name the version it read. `API_SPEC` section 17 keeps the version source, the required header, and the two result codes together: a missing `If-Match` answers `42801`, a stale one answers `41201`. */
  version: string;
}
