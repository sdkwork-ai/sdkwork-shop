import type { ShopReturnAddressResponse } from './shop-return-address-response';

export interface CurrentReturnAddressesUpdateResponse {
  code: 0;
  data: unknown & { item: ShopReturnAddressResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
