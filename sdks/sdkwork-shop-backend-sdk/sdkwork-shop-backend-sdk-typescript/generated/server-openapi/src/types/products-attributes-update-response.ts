import type { ProductAttributes } from './product-attributes';

export interface ProductsAttributesUpdateResponse {
  code: 0;
  data: unknown & { item: ProductAttributes; };
  /** Server-owned request correlation id. */
  traceId: string;
}
