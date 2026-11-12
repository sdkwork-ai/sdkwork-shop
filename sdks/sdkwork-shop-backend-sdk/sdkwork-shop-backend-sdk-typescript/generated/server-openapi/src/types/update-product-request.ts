export interface UpdateProductRequest {
  /** New default-locale title. Bound by ck_commerce_product_spu_name_length (1..300) through the derived name. */
  title?: string;
  /** New default-locale subtitle. */
  subtitle?: string;
  /** New default-locale description. Three-state on purpose: omitting the field leaves the stored text unchanged, an explicit `null` clears it, and a value replaces it. Clearing is stated rather than inferred because description is a nullable column — NULL is how a product is stored with no description, so an absent key must not be read as a request to remove one. */
  description?: string | null;
  /** Move the product to another category. */
  categoryId?: string;
  /** Lifecycle status. ck_commerce_product_spu_status admits these four, and salesStatus is derived from the value on every write. draft is a state to leave, not one to return to: ck_commerce_product_spu_published_at refuses status=draft once publishedAt is recorded, because publishing is a one-way transition. Retirement is a DELETE. */
  status?: 'draft' | 'active' | 'inactive' | 'archived';
  /** Three-state. Absent leaves the stored model number alone; an explicit null removes it; a string replaces it. */
  modelNo?: string | null;
  /** Three-state. A restated barcode must be unique per tenant (uk_commerce_product_spu_tenant_barcode). */
  barcode?: string | null;
  /** Three-state. ck_commerce_product_spu_weight pins a restated value >= 0. */
  weightGram?: string | null;
  /** Three-state. ck_commerce_product_spu_volume pins a restated value >= 0. */
  volumeMl?: string | null;
  /** Three-state. */
  taxClassCode?: string | null;
  /** Three-state. Absent keeps the stored brand; an explicit null detaches the product from its brand; an id moves it. */
  brandId?: string | null;
  /** The locale the text fields are written in. Omitted — or the default language — writes the base row; any other locale writes a translation row and leaves the base row alone. */
  locale?: string;
}
