export interface CategoryAttribute {
  /** Primary key of the binding. */
  id: string;
  /** Tenant that owns the binding. Always the calling scope. */
  tenantId: string;
  /** Organization scope of the binding. `0` is the tenant-wide sentinel. */
  organizationId: string;
  /** Category the attribute is bound to. */
  categoryId: string;
  /** Attribute bound to the category. */
  attributeId: string;
  /** Role of the attribute inside this category. ck_commerce_product_category_attribute_role is the authority for this set. */
  attributeRole: 'key' | 'sales' | 'parameter';
  /** Category the binding was inherited from, when it was not authored directly on `categoryId`. Never equal to `categoryId` (ck_commerce_product_category_attribute_source_not_self). */
  sourceCategoryId?: string | null;
  /** Whether a product in this category must carry the attribute. */
  required: boolean;
  /** Whether the attribute contributes to free-text search. */
  searchable: boolean;
  /** Whether product lists may filter on the attribute. */
  filterable: boolean;
  /** Whether the attribute may be used to compare products. */
  comparable: boolean;
  /** Display ordering key among the category bindings, ascending. */
  sortOrder: string;
  /** Shared lifecycle vocabulary. The authority for this set is the column CHECK constraint named in the field description. */
  status: 'active' | 'inactive';
  /** Row creation time. */
  createdAt: string;
  /** Last write time. */
  updatedAt: string;
  /** Row version, advanced by every write to this row and by any write that rewrites the row's own storage identity (a category move also advances the moved subtree). It is the value an update or delete must send back as `If-Match`, and the same number the response's `ETag` carries, so a client that has read a resource can always name the version it read. `API_SPEC` section 17 keeps the version source, the required header, and the two result codes together: a missing `If-Match` answers `42801`, a stale one answers `41201`. */
  version: string;
}
