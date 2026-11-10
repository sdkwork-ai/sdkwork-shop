export interface CreatePriceListItemRequest {
  /** SKU the step prices. Must name a live SKU of the tenant. */
  skuId: string;
  /** The list to price into. The currency and scale are inherited from it. */
  listId: string;
  /** The price at this step, in the list's minor unit. API_SPEC section 13.2.1 fixes the wire unit at the minor unit. */
  priceMinor: string;
  /** The quantity this step starts at. Defaults to 1, the first rung of the ladder. */
  minQuantity?: string;
}
