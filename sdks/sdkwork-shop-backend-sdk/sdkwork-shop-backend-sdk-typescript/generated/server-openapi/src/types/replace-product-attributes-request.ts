import type { ProductAttributeValueInput } from './product-attribute-value-input';

/** Whole-set replacement, not patching: the detail page renders the set as one table, and a partial edit cannot express "this product no longer lists a warranty term". One live value per attribute (uk_commerce_product_spu_attribute_value); each entry carries exactly one of attributeValueId or rawValue (ck_commerce_product_spu_attribute_carrier). */
export interface ReplaceProductAttributesRequest {
  /** The product's whole key/parameter attribute set, replacing whatever was stored. An empty array clears the set. */
  attributes: ProductAttributeValueInput[];
}
