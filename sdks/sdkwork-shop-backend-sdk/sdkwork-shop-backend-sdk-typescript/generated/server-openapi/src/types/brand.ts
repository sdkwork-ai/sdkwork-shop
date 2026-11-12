export interface Brand {
  /** Primary key. */
  id: string;
  /** The tenant the brand belongs to. Always the caller's own scope. */
  tenantId: string;
  /** The organization the brand belongs to. Always the caller's own scope. */
  organizationId: string;
  /** Stable business key of the brand, unique per tenant while the row is live (uk_commerce_product_brand_tenant_no). */
  brandNo: string;
  /** Default-locale display name. Bounded by ck_commerce_product_brand_name_length (1..200). */
  name: string;
  /** Default-locale description of the brand. */
  description?: string | null;
  /** Shared lifecycle vocabulary. The authority for this set is ck_commerce_product_brand_status. */
  status: 'active' | 'inactive';
  /** Sibling ordering key, ascending. */
  sortOrder: string;
  /** Row creation time. */
  createdAt: string;
  /** Last write time. */
  updatedAt: string;
  /** Row version, advanced by every write to this row and by any write that retires it. Echo it back as `If-Match`. */
  version: string;
}
