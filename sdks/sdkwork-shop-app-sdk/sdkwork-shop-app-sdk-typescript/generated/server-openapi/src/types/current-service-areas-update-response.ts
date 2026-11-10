import type { ShopServiceAreaResponse } from './shop-service-area-response';

export interface CurrentServiceAreasUpdateResponse {
  code: 0;
  data: unknown & { item: ShopServiceAreaResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
