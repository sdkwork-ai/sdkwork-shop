import type { MediaResource } from './media-resource';

export interface Media {
  /** Primary key of the attachment. */
  id: string;
  /** Kind of row the attachment hangs off. The values are the storage vocabulary the `ck_commerce_product_media_owner_type` CHECK accepts; this surface calls the `spu` owner a *product* everywhere else (`/catalog/products/{productId}`), but the filter value stays `spu`. */
  ownerType: 'spu' | 'sku' | 'category' | 'attribute_value';
  /** Id of the owning row, in the vocabulary `ownerType` names. */
  ownerId: string;
  /** Role the resource plays on this owner. MEDIA_RESOURCE_SPEC section 5 fixes this set for the merchandise profile, and `ck_commerce_product_media_owner_role` narrows it for one owner kind: an `attribute_value` owner carries a swatch, so only `main_image`, `sku_image`, and `gallery_image` are accepted there. A role outside that subset is refused as a validation error, not by the column. */
  mediaRole: 'main_image' | 'gallery_image' | 'detail_image' | 'sku_image' | 'video' | 'manual' | 'certificate';
  /** Drive node id the attachment references. It always equals `resourceSnapshot.id`: both are written from the same request field, and the reference is derived from the snapshot rather than accepted alongside it. */
  mediaResourceId: string;
  resourceSnapshot: MediaResource;
  /** Attachment-level accessible description; null when the resource snapshot carries it. */
  altText?: string | null;
  /** Ordering rank within the owner-and-role slot. */
  sortOrder: string;
  /** Lifecycle status. ck_commerce_product_media_status is the authority for this set. */
  status: 'active' | 'inactive';
  /** Row creation time. */
  createdAt: string;
  /** Last write time. */
  updatedAt: string;
  /** Row version, advanced by every write to this row and by any write that rewrites the row's own storage identity (a category move also advances the moved subtree). It is the value an update or delete must send back as `If-Match`, and the same number the response's `ETag` carries, so a client that has read a resource can always name the version it read. `API_SPEC` section 17 keeps the version source, the required header, and the two result codes together: a missing `If-Match` answers `42801`, a stale one answers `41201`. */
  version: string;
}
