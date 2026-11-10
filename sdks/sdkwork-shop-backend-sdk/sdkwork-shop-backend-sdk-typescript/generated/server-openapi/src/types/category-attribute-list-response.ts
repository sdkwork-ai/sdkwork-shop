import type { CategoryAttribute } from './category-attribute';
import type { PageInfo } from './page-info';

export interface CategoryAttributeListResponse {
  code: 0;
  data: unknown & { items: CategoryAttribute[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
