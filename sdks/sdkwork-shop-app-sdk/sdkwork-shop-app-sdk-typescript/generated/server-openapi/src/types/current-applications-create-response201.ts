import type { ShopApplicationResponse } from './shop-application-response';

export interface CurrentApplicationsCreateResponse201 {
  code: 0;
  data: unknown & { item: ShopApplicationResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
