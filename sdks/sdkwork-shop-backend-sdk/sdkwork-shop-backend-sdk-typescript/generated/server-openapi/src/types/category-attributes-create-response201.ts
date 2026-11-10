import type { CategoryAttribute } from './category-attribute';

export interface CategoryAttributesCreateResponse201 {
  code: 0;
  data: unknown & { item: CategoryAttribute; };
  /** Server-owned request correlation id. */
  traceId: string;
}
