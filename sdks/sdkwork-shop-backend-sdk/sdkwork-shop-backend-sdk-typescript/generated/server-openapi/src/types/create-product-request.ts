/** The fixed-semantics fields (model number, barcode, weight, volume, tax class) are columns, not dynamic attributes, so they are queryable and constrainable — the baseline's own comment on commerce_product_spu. */
export interface CreateProductRequest {
  /** Stable business key of the product, unique per tenant while the row is live (uk_commerce_product_spu_tenant_no). */
  productNo: string;
  /** Default-locale display title. commerce_product_spu.name is derived from it and bound by ck_commerce_product_spu_name_length (1..300). */
  title: string;
  /** Optional default-locale subtitle. */
  subtitle?: string;
  /** Optional default-locale description. */
  description?: string;
  /** How the product is delivered in the large. ck_commerce_product_spu_product_type is the authority for this set. */
  productType: 'physical' | 'digital' | 'service' | 'membership' | 'points';
  /** Category the product belongs to. Required: commerce_product_spu.category_id is NOT NULL. */
  categoryId: string;
  /** The manufacturer's standard model number, when one is declared. */
  modelNo?: string;
  /** The product-level barcode (GTIN/EAN), unique per tenant while the row is live. */
  barcode?: string;
  /** Ship weight in grams, the figure logistics rates a shipment from. */
  weightGram?: string;
  /** Shipping volume in millilitres. */
  volumeMl?: string;
  /** The tenant's tax-classification key for this product. */
  taxClassCode?: string;
  /** The brand this product belongs to, when one is declared. */
  brandId?: string;
  /** The locale the text fields are written in. Omitted — or the default language — writes the base row; any other locale writes a translation row and leaves the base row alone. */
  locale?: string;
}
