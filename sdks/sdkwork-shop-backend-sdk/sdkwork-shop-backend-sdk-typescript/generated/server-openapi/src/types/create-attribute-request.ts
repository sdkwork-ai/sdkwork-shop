export interface CreateAttributeRequest {
  /** Stable business key of the attribute, unique per tenant while the row is live (uk_commerce_product_attribute_tenant_no). */
  attributeNo: string;
  /** Default-locale display name. Bound by ck_commerce_product_attribute_name_length (1..100). */
  name: string;
  /** Declared value type. `enum` selects from the dictionary this attribute owns; the other four accept typed free input. `ck_commerce_product_attribute_value_type` is the authority for this set. A type is a create-time fact: it has no update path, because changing it after values exist would strand every stored value that no longer fits. */
  valueType: 'enum' | 'text' | 'number' | 'bool' | 'date';
  /** The control a console renders for this attribute. `ck_commerce_product_attribute_input_hint` is the authority for this set. */
  inputHint: 'select' | 'multi_select' | 'text' | 'number' | 'toggle' | 'date';
  /** Whether the attribute accepts more than one dictionary value per product. Requires valueType `enum` (ck_commerce_product_attribute_multi_value_needs_enum). Create-time fact, like the type itself. */
  isMultiValue?: boolean;
  /** The unit the attribute's values are denominated in (`cm`, `kg`), when one is declared. */
  unitSymbol?: string;
  /** Sibling ordering key, ascending. Defaults to 0. */
  sortOrder?: string;
  /** Optional initial enumeration. Each entry becomes one commerce_product_attribute_value row. Only valid for valueType `enum`: a free-text attribute has no dictionary to seed. */
  values?: string[];
  /** The locale the text fields are written in. Omitted — or the default language — writes the base row; any other locale writes a translation row and leaves the base row alone. */
  locale?: string;
}
