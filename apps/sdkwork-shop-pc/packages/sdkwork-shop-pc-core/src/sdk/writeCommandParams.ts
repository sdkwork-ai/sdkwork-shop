import { uuid } from '@sdkwork/utils/id';

export interface SdkworkWriteCommandParams {
  idempotencyKey: string;
  sdkworkRequestHash: string;
  xIdempotencyFingerprint: string;
}

function normalizeRequestHashPart(part: string): string {
  return part
    .split('')
    .map((character) => (
      /^[a-zA-Z0-9._-]$/u.test(character) ? character : '-'
    ))
    .join('');
}

function canonicalJsonString(value: unknown): string {
  if (value === null || value === undefined) {
    return 'null';
  }
  if (typeof value === 'boolean' || typeof value === 'number') {
    return String(value);
  }
  if (typeof value === 'string') {
    return JSON.stringify(value);
  }
  if (Array.isArray(value)) {
    return `[${value.map((entry) => canonicalJsonString(entry)).join(',')}]`;
  }
  if (typeof value === 'object') {
    const record = value as Record<string, unknown>;
    const entries = Object.keys(record)
      .sort()
      .filter((key) => record[key] !== null && record[key] !== undefined)
      .map((key) => `${JSON.stringify(key)}:${canonicalJsonString(record[key])}`);
    return `{${entries.join(',')}}`;
  }
  return JSON.stringify(value);
}

export function createSdkworkWriteCommandParams(
  scope: string,
  payload: unknown,
  idempotencyKey = uuid(),
): SdkworkWriteCommandParams {
  const sdkworkRequestHash = [scope, canonicalJsonString(payload)]
    .map(normalizeRequestHashPart)
    .join('-');
  return {
    idempotencyKey,
    sdkworkRequestHash,
    xIdempotencyFingerprint: sdkworkRequestHash,
  };
}
