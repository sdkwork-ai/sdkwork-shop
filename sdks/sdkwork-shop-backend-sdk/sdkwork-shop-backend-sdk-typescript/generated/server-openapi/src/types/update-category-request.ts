export interface UpdateCategoryRequest {
  /** New parent. Omit to leave the parent unchanged. */
  parentId?: string;
  /** New default-locale name. Bound by ck_commerce_product_category_name_length (1..200). */
  name?: string;
  /** New ordering rank. */
  sortOrder?: string;
  /** Lifecycle status. ck_commerce_product_category_status admits 'active' and 'inactive' only; retirement is a DELETE, never a status. */
  status?: 'active' | 'inactive';
  /** Three-state. Absent keeps the stored back-office position; an explicit null removes the category from the back-office tree; an id moves it. */
  backParentId?: string | null;
}
