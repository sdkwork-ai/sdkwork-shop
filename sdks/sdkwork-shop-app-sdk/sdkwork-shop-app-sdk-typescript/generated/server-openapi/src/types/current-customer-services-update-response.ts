import type { ShopCustomerServiceResponse } from './shop-customer-service-response';

export interface CurrentCustomerServicesUpdateResponse {
  code: 0;
  data: unknown & { item: ShopCustomerServiceResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
