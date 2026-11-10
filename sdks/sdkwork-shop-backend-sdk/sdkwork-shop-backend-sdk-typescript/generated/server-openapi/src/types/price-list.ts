export interface PriceList {
  /** Primary key. */
  id: string;
  /** Tenant that owns the price list. Always the calling scope. */
  tenantId: string;
  /** Organization scope of the price list. `0` is the tenant-wide sentinel. */
  organizationId: string;
  /** Stable business key of the price list, unique per tenant while the row is live. */
  priceListNo: string;
  /** Display name. Bounded by ck_commerce_price_list_name_length. */
  name: string;
  /** ISO 4217 code every amount in this price list is stated in. */
  currencyCode: string;
  /** The currency's display symbol, resolved from the currency table at read time. */
  currencySymbol: string;
  /** Market the price list targets, when it is market-scoped. */
  marketCode?: string | null;
  /** The B2B customer segment the list applies to, when it is segment-scoped. */
  customerSegment?: string | null;
  /** Shared lifecycle vocabulary. The authority for this set is the column CHECK constraint named in the field description. */
  status: 'active' | 'inactive';
  /** Window start. `endsAt` must be after `startsAt` when both are set (ck_commerce_price_list_window). */
  startsAt?: string | null;
  /** Window end. */
  endsAt?: string | null;
  /** Row creation time. */
  createdAt: string;
  /** Last write time. */
  updatedAt: string;
  /** Row version, advanced by every write to this row and by any write that rewrites the row's own storage identity (a category move also advances the moved subtree). It is the value an update or delete must send back as `If-Match`, and the same number the response's `ETag` carries, so a client that has read a resource can always name the version it read. `API_SPEC` section 17 keeps the version source, the required header, and the two result codes together: a missing `If-Match` answers `42801`, a stale one answers `41201`. */
  version: string;
}
