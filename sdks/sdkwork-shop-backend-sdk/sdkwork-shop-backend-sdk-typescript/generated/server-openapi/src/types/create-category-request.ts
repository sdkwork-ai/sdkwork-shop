export interface CreateCategoryRequest {
  /** Stable business key of the category, unique per tenant while the row is live (uk_commerce_product_category_tenant_no). */
  categoryNo: string;
  /** Parent category id. Omit for a root category. */
  parentId?: string;
  /** Default-locale display name. Bound by ck_commerce_product_category_name_length (1..200). */
  name: string;
  /** Ordering rank among the siblings. Defaults to 0. */
  sortOrder?: string;
  /** The back-office tree position, when the category sits on one. The two trees are independent. */
  backParentId?: string;
}
