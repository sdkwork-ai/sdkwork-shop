export interface UpdatePriceListRequest {
  /** Lifecycle status. ck_commerce_price_list_status admits 'active' and 'inactive'. */
  status?: 'active' | 'inactive';
  /** Inclusive start of the window in which the list applies. Omit for an open start. */
  startsAt?: string;
  /** Exclusive end of the window. Must be after startsAt when both are stated (ck_commerce_price_list_window). Omit for an open end. */
  endsAt?: string;
  /** Replacement display name. Bound by ck_commerce_price_list_name_length. */
  name?: string;
  /** Three-state. Absent leaves the stored segment alone; an explicit null detaches the list from any segment; a string replaces it. */
  customerSegment?: string | null;
}
