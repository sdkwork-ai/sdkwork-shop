/** Canonical media resource (MEDIA_RESOURCE_SPEC section 3). The same schema is used for the request field that attaches a resource and for the `resourceSnapshot` a stored attachment publishes, so a caller writes and reads one shape. `id` is required here, unlike the base model: `commerce_product_media.media_resource_id` is `BIGINT NOT NULL`, so a resource with no Drive identity has no storage form on this surface. */
export interface MediaResource {
  /** Stable Drive node id of the resource (MEDIA_RESOURCE_SPEC section 5.1). It is the reference `commerce_product_media.media_resource_id` stores, and the same `source = drive` resource may be attached to several owners. */
  id: string;
  /** Media kind. MEDIA_RESOURCE_SPEC section 3 fixes this set. */
  kind: 'image' | 'video' | 'audio' | 'voice' | 'document' | 'archive' | 'model' | 'other';
  /** Where the bytes live. An SDKWork-owned file must use `drive`; `data_url` is only accepted on a transient payload and must never be persisted as business state. */
  source: 'drive' | 'external_url' | 'data_url' | 'provider_asset' | 'generated';
  /** Delivery URL. It is a hint, not the system of record: it may be signed, temporary, or CDN-derived, which is why no column stores it. */
  url?: string;
  publicUrl?: string;
  /** Stable URI of the resource. For a Drive resource it is `drive://spaces/{spaceId}/nodes/{nodeId}` and must agree with `id`. */
  uri?: string;
  objectBlobId?: string;
  fileName?: string;
  mimeType?: string;
  sizeBytes?: string;
  checksum?: { algorithm: 'sha256' | 'md5' | 'etag'; value: string; };
  width?: number;
  height?: number;
  durationSeconds?: number;
  altText?: string;
  title?: string;
  poster?: MediaResource;
  thumbnails?: MediaResource[];
  variants?: MediaResource[];
  access?: { visibility: 'private' | 'tenant' | 'organization' | 'public' | 'signed'; expiresAt?: string; };
  ai?: { provenance?: 'uploaded' | 'generated' | 'edited' | 'imported'; provider?: string; model?: string; promptId?: string; generationTaskId?: string; sourceMediaIds?: string[]; seed?: string; moderationStatus?: 'unknown' | 'pending' | 'approved' | 'rejected' | 'blocked'; safetyLabels?: string[]; };
  metadata?: Record<string, unknown>;
}
