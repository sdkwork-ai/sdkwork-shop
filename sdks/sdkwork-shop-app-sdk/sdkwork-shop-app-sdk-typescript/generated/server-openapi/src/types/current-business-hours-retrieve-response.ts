import type { ShopBusinessHourResponse } from './shop-business-hour-response';

export interface CurrentBusinessHoursRetrieveResponse {
  code: 0;
  data: unknown & { item: ShopBusinessHourResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
