import type { PageInfo } from './page-info';
import type { PriceListItem } from './price-list-item';

export interface PriceListItemListResponse {
  code: 0;
  data: unknown & { items: PriceListItem[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
