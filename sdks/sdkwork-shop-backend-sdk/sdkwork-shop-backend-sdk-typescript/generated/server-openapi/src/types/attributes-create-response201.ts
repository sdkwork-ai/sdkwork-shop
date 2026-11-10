import type { Attribute } from './attribute';

export interface AttributesCreateResponse201 {
  code: 0;
  data: unknown & { item: Attribute; };
  /** Server-owned request correlation id. */
  traceId: string;
}
