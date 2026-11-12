export interface CreateBrandRequest {
  /** Stable business key of the brand, unique per tenant while the row is live. */
  brandNo: string;
  /** Default-locale display name. Bound by ck_commerce_product_brand_name_length (1..200). */
  name: string;
  /** Default-locale description of the brand. */
  description?: string;
  /** Sibling ordering key, ascending. Defaults to 0. */
  sortOrder?: string;
  /** The locale the text fields are written in. Omitted — or the default language — writes the base row; any other locale writes a translation row and leaves the base row alone. */
  locale?: string;
}
