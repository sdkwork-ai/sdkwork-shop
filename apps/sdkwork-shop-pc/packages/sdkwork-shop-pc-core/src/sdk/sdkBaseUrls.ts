import {resolveBaseUrlWithAlignProtocol} from '@sdkwork/sdk-common';

export function resolveCommerceAppSdkBaseUrl(): string {
  // Single shared base-url key; the matching API host is chosen from the
  // current page's environment+brand. These commerce SDK clients expect a
  // bare origin (no /app/v3/api path suffix).
  return resolveBaseUrlWithAlignProtocol({ envKey: 'SDKWORK_API_BASE_URL' }).url;
}
