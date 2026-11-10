import type { PageInfo } from './page-info';
import type { Product } from './product';

export interface ProductListResponse {
  code: 0;
  data: unknown & { items: Product[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
