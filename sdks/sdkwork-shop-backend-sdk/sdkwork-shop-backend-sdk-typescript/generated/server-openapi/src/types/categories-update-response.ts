import type { Category } from './category';

export interface CategoriesUpdateResponse {
  code: 0;
  data: unknown & { item: Category; };
  /** Server-owned request correlation id. */
  traceId: string;
}
