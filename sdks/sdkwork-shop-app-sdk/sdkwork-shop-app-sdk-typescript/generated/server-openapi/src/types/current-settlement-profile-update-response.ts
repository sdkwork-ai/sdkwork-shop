import type { ShopSettlementProfileResponse } from './shop-settlement-profile-response';

export interface CurrentSettlementProfileUpdateResponse {
  code: 0;
  data: unknown & { item: ShopSettlementProfileResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
