export interface UpdateCategoryAttributeRequest {
  /** Reclassify the binding. Changing a role is a normal merchandising edit. */
  role?: 'key' | 'sales' | 'parameter';
  /** New requiredness. */
  required?: boolean;
  /** New searchability. */
  searchable?: boolean;
  /** New filterability. */
  filterable?: boolean;
  /** New comparability. */
  comparable?: boolean;
  /** New ordering rank. */
  sortOrder?: string;
  /** Lifecycle status of the binding. ck_commerce_product_category_attribute_status admits 'active' and 'inactive'. */
  status?: 'active' | 'inactive';
}
