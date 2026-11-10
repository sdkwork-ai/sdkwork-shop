/** One price list. The list is the currency authority for its items: every row of the list inherits this currency_code and the currency's exponent. */
export interface CreatePriceListRequest {
  /** Stable business key of the price list, unique per tenant while the row is live. commerce_price_list.name is derived from it and bound by ck_commerce_price_list_name_length (1..200). */
  priceListNo: string;
  /** ISO 4217 code of the currency every amount in this request is stated in. Bound by ck_commerce_currency_code_shape and by the foreign key to commerce_currency. */
  currencyCode: string;
  /** Market the list applies to, when it is market-scoped. Omit for a list that applies to every market. */
  marketCode?: string;
  /** Human display name of the list. Bound by ck_commerce_price_list_name_length (1..200). */
  name: string;
  /** The B2B customer segment the list applies to, when it is segment-scoped. */
  customerSegment?: string;
}
