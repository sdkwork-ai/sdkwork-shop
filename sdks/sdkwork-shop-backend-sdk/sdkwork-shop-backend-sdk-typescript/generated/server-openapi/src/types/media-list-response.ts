import type { Media } from './media';
import type { PageInfo } from './page-info';

export interface MediaListResponse {
  code: 0;
  data: unknown & { items: Media[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
