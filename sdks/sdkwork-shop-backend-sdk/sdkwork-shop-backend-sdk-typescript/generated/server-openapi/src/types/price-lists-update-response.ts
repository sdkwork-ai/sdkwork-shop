import type { PriceList } from './price-list';

export interface PriceListsUpdateResponse {
  code: 0;
  data: unknown & { item: PriceList; };
  /** Server-owned request correlation id. */
  traceId: string;
}
