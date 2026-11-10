import type { Attribute } from './attribute';

export interface AttributesUpdateResponse {
  code: 0;
  data: unknown & { item: Attribute; };
  /** Server-owned request correlation id. */
  traceId: string;
}
