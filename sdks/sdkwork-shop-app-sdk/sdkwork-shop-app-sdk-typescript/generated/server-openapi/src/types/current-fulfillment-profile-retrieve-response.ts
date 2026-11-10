import type { ShopFulfillmentProfileResponse } from './shop-fulfillment-profile-response';

export interface CurrentFulfillmentProfileRetrieveResponse {
  code: 0;
  data: unknown & { item: ShopFulfillmentProfileResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
