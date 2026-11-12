export interface UpdateBrandRequest {
  /** Replacement display name. Bound by ck_commerce_product_brand_name_length (1..200). */
  name?: string;
  /** Three-state. Absent leaves the stored text alone; an explicit null removes it; a string replaces it. */
  description?: string | null;
  /** Sibling ordering key, ascending. */
  sortOrder?: string;
  /** Shared lifecycle vocabulary. The authority for this set is ck_commerce_product_brand_status. */
  status?: 'active' | 'inactive';
  /** The locale the text fields are written in. Omitted — or the default language — writes the base row; any other locale writes a translation row and leaves the base row alone. */
  locale?: string;
}
