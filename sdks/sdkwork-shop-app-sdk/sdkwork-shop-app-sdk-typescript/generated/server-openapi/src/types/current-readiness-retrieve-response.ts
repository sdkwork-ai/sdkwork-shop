import type { ShopReadinessResponse } from './shop-readiness-response';

export interface CurrentReadinessRetrieveResponse {
  code: 0;
  data: unknown & { item: ShopReadinessResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
