import type { Media } from './media';

export interface MediaUpdateResponse {
  code: 0;
  data: unknown & { item: Media; };
  /** Server-owned request correlation id. */
  traceId: string;
}
