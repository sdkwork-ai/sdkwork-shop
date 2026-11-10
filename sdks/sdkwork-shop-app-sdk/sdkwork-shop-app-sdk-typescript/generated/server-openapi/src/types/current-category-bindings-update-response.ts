import type { ShopCategoryBindingResponse } from './shop-category-binding-response';

export interface CurrentCategoryBindingsUpdateResponse {
  code: 0;
  data: unknown & { item: ShopCategoryBindingResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
