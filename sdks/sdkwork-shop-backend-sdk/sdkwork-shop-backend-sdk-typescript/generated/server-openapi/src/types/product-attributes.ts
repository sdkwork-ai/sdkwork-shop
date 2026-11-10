/** The product's whole key/parameter attribute set as stored after a replacement. This is the specification table a product detail page renders; replace it wholesale with PUT on the sub-collection. */
export interface ProductAttributes {
  /** Always present, empty when the product declares no key/parameter attributes. This is the specification table a product detail page renders. Replace it wholesale with PUT /backend/v3/api/catalog/products/{productId}/attributes. */
  attributes: ({ attributeId: string; attributeNo: string; attributeName: string; attributeRole: 'key' | 'parameter'; attributeValueId?: string | null; displayValue?: string | null; rawValue?: string | null; sortOrder: string; })[];
  /** The product row's version after the replacement. The set is part of the product aggregate, so this is the number the next If-Match on the product (or on this set) must name; it is the same value the response's ETag carries. */
  version: string;
}
