import type { ShopBrandAuthorizationResponse } from './shop-brand-authorization-response';

export interface CurrentBrandAuthorizationsUpdateResponse {
  code: 0;
  data: unknown & { item: ShopBrandAuthorizationResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
