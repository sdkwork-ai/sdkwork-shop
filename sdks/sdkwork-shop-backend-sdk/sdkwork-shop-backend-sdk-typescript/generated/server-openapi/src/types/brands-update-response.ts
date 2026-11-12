import type { Brand } from './brand';

export interface BrandsUpdateResponse {
  code: 0;
  data: unknown & { item: Brand; };
  /** Server-owned request correlation id. */
  traceId: string;
}
