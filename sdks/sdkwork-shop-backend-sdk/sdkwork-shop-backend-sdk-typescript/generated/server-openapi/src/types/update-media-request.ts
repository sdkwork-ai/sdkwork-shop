import type { MediaResource } from './media-resource';

export interface UpdateMediaRequest {
  /** Role the resource plays on this owner. MEDIA_RESOURCE_SPEC section 5 fixes this set for the merchandise profile, and `ck_commerce_product_media_owner_role` narrows it for one owner kind: an `attribute_value` owner carries a swatch, so only `main_image`, `sku_image`, and `gallery_image` are accepted there. A role outside that subset is refused as a validation error, not by the column. */
  mediaRole?: 'main_image' | 'gallery_image' | 'detail_image' | 'sku_image' | 'video' | 'manual' | 'certificate';
  resource?: MediaResource;
  altText?: string;
  /** New ordering rank. The slot unique key is partial on `deleted_at IS NULL`, so two live attachments of one role cannot share a rank. */
  sortOrder?: string;
  /** Lifecycle status. ck_commerce_product_media_status admits `active` and `inactive`; retirement is the delete operation, which keeps the row for history. */
  status?: 'active' | 'inactive';
}
