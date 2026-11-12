import type { Brand } from './brand';

export interface BrandsRetrieveResponse {
  code: 0;
  data: unknown & { item: Brand; };
  /** Server-owned request correlation id. */
  traceId: string;
}
