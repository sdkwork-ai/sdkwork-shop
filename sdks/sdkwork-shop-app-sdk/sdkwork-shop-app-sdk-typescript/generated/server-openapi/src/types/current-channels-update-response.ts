import type { ShopChannelResponse } from './shop-channel-response';

export interface CurrentChannelsUpdateResponse {
  code: 0;
  data: unknown & { item: ShopChannelResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
