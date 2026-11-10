import type { Sku } from './sku';

export interface SkusCreateResponse201 {
  code: 0;
  data: unknown & { item: Sku; };
  /** Server-owned request correlation id. */
  traceId: string;
}
