import type { Attribute } from './attribute';
import type { PageInfo } from './page-info';

export interface AttributeListResponse {
  code: 0;
  data: unknown & { items: Attribute[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
