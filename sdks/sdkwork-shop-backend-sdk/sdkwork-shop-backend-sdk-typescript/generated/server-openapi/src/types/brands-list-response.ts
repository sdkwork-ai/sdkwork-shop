import type { Brand } from './brand';
import type { PageInfo } from './page-info';

export interface BrandsListResponse {
  code: 0;
  data: unknown & { items: Brand[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
