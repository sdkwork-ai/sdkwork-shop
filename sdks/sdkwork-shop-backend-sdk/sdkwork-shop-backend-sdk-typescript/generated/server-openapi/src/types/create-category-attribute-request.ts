export interface CreateCategoryAttributeRequest {
  /** Category whose template the attribute joins. */
  categoryId: string;
  /** Attribute dictionary entry being bound. */
  attributeId: string;
  /** What the attribute means in THIS category: a key attribute identifies the product, a sales attribute splits it into SKUs, a parameter only describes it. Defaults to 'parameter' so an unclassified binding cannot accidentally declare a sales axis. ck_commerce_product_category_attribute_role is the authority for this set. */
  role?: 'key' | 'sales' | 'parameter';
  /** Whether a product in this category must carry the attribute. */
  required?: boolean;
  /** Whether the value is indexed for search. */
  searchable?: boolean;
  /** Whether the value is offered as a list filter. */
  filterable?: boolean;
  /** Whether the value is offered in comparisons. */
  comparable?: boolean;
  /** Category this binding was inherited from, when it was materialised rather than authored here. Must differ from categoryId (ck_commerce_product_category_attribute_source_not_self). */
  sourceCategoryId?: string;
  /** Ordering rank inside the template. Defaults to 0. */
  sortOrder?: string;
}
