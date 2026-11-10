import type { ShopPolicyResponse } from './shop-policy-response';

export interface CurrentPoliciesUpdateResponse {
  code: 0;
  data: unknown & { item: ShopPolicyResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
