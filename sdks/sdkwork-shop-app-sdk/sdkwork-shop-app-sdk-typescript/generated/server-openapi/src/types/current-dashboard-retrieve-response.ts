import type { ShopDashboardResponse } from './shop-dashboard-response';

export interface CurrentDashboardRetrieveResponse {
  code: 0;
  data: unknown & { item: ShopDashboardResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
