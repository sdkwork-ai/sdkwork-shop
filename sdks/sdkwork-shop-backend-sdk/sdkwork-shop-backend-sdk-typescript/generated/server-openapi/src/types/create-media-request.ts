import type { MediaResource } from './media-resource';

export interface CreateMediaRequest {
  /** Kind of row the attachment hangs off. The values are the storage vocabulary the `ck_commerce_product_media_owner_type` CHECK accepts; this surface calls the `spu` owner a *product* everywhere else (`/catalog/products/{productId}`), but the filter value stays `spu`. */
  ownerType: 'spu' | 'sku' | 'category' | 'attribute_value';
  /** Id of the owning row. Together with `ownerType` it addresses exactly one row; the pair is not constrained by a foreign key, because PostgreSQL cannot express "this BIGINT points at one of four tables", so the server verifies the owner exists for the declared kind inside the same transaction that inserts the attachment. */
  ownerId: string;
  /** Role the resource plays on this owner. MEDIA_RESOURCE_SPEC section 5 fixes this set for the merchandise profile, and `ck_commerce_product_media_owner_role` narrows it for one owner kind: an `attribute_value` owner carries a swatch, so only `main_image`, `sku_image`, and `gallery_image` are accepted there. A role outside that subset is refused as a validation error, not by the column. */
  mediaRole: 'main_image' | 'gallery_image' | 'detail_image' | 'sku_image' | 'video' | 'manual' | 'certificate';
  resource: MediaResource;
  /** Accessible description for the attachment, overriding the resource `altText`. */
  altText?: string;
  /** Ordering rank within `(ownerType, ownerId, mediaRole)`, ascending. It is unique inside that slot while the row is live (uk_commerce_product_media_slot) and never negative (ck_commerce_product_media_sort_order). The lowest rank of a role is the one rendered first. */
  sortOrder?: string;
}
