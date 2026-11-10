import type { Media } from './media';

export interface MediaCreateResponse201 {
  code: 0;
  data: unknown & { item: Media; };
  /** Server-owned request correlation id. */
  traceId: string;
}
