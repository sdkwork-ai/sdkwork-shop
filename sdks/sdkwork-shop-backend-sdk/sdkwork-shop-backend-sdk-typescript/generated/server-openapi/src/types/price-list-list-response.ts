import type { PageInfo } from './page-info';
import type { PriceList } from './price-list';

export interface PriceListListResponse {
  code: 0;
  data: unknown & { items: PriceList[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
