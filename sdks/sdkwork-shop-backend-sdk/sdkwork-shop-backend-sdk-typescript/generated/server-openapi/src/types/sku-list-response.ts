import type { PageInfo } from './page-info';
import type { Sku } from './sku';

export interface SkuListResponse {
  code: 0;
  data: unknown & { items: Sku[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
