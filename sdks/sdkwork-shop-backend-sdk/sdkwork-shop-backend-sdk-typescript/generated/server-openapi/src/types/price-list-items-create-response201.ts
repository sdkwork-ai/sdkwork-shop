import type { PriceListItem } from './price-list-item';

export interface PriceListItemsCreateResponse201 {
  code: 0;
  data: unknown & { item: PriceListItem; };
  /** Server-owned request correlation id. */
  traceId: string;
}
