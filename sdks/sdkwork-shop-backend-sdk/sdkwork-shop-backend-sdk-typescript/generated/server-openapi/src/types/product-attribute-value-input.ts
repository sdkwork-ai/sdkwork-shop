export interface ProductAttributeValueInput {
  /** The attribute this entry states. It must be bound — active — in the product's category template with the same role. */
  attributeId: string;
  /** The role the entry takes on the product. Only `key` and `parameter` are legal here: the buyer-selectable combinations live on the SKU (ck_commerce_product_spu_attribute_role). The role must match the category template's binding. */
  role: 'key' | 'parameter';
  /** The dictionary value the entry carries. Required for an `enum` attribute and refused for any other type; exactly one carrier per entry. */
  attributeValueId?: string;
  /** The free-text value the entry carries. Required for a non-`enum` attribute and refused for an `enum` one; exactly one carrier per entry. */
  rawValue?: string;
  /** Sibling ordering key, ascending. Defaults to 0. */
  sortOrder?: string;
}
