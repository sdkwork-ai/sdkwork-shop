import type { Category } from './category';
import type { PageInfo } from './page-info';

export interface CategoryListResponse {
  code: 0;
  data: unknown & { items: Category[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
