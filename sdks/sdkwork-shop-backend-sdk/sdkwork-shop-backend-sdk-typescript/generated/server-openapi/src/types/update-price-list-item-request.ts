export interface UpdatePriceListItemRequest {
  /** New price at this step. API_SPEC section 13.2.1 fixes the wire unit at the minor unit. */
  priceMinor?: string;
  /** New tier start. Restating it onto a step another row of the same list already prices is a `409`, not a silent merge. */
  minQuantity?: string;
  /** Shared lifecycle status. */
  status?: 'active' | 'inactive';
}
