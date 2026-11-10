import type { AttributeValue } from './attribute-value';
import type { PageInfo } from './page-info';

export interface AttributeValuesListResponse {
  code: 0;
  data: unknown & { items: AttributeValue[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
