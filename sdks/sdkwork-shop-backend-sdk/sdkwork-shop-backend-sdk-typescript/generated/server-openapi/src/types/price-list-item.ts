export interface PriceListItem {
  /** Primary key. */
  id: string;
  /** The price list this row prices into. */
  listId: string;
  /** SKU the ladder step prices. */
  skuId: string;
  /** Inherited from the list; the composite foreign key refuses an item quoting money its list does not deal in. */
  currencyCode: string;
  /** A unit exponent, not an amount: the list's price scale the minor-unit amounts are stated against. */
  minorUnitExponent: string;
  /** The price at this ladder step, in the currency's minor unit. ck_commerce_price_list_item_price keeps it non-negative. API_SPEC section 13.2.1 fixes the wire unit at the minor unit. */
  priceMinor: string;
  /** The quantity this step starts at; 1 is the first rung of the ladder (ck_commerce_price_list_item_min_quantity). */
  minQuantity: string;
  /** Shared lifecycle status; ck_commerce_price_list_item_status admits these two. */
  status: 'active' | 'inactive';
  /** Row creation instant. */
  createdAt: string;
  /** Row's last write instant. */
  updatedAt: string;
  /** Monotonic row version, bumped by every write. Echo it back as `If-Match` on the next update or delete. */
  version: string;
}
