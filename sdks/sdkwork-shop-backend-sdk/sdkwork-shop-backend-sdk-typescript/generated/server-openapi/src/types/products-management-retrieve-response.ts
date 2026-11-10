import type { Product } from './product';

export interface ProductsManagementRetrieveResponse {
  code: 0;
  data: unknown & { item: Product; };
  /** Server-owned request correlation id. */
  traceId: string;
}
