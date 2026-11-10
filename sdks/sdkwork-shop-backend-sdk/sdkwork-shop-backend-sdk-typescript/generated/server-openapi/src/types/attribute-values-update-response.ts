import type { AttributeValue } from './attribute-value';

export interface AttributeValuesUpdateResponse {
  code: 0;
  data: unknown & { item: AttributeValue; };
  /** Server-owned request correlation id. */
  traceId: string;
}
