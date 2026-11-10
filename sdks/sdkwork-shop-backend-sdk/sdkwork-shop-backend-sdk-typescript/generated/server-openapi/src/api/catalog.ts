import { backendApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { Attribute, AttributeValue, Category, CategoryAttribute, CreateAttributeRequest, CreateAttributeValueRequest, CreateCategoryAttributeRequest, CreateCategoryRequest, CreateMediaRequest, CreatePriceListItemRequest, CreatePriceListRequest, CreateProductRequest, CreateSkuRequest, Media, PageInfo, PriceList, PriceListItem, Product, ProductAttributes, ReplaceProductAttributesRequest, Sku, UpdateAttributeRequest, UpdateAttributeValueRequest, UpdateCategoryAttributeRequest, UpdateCategoryRequest, UpdateMediaRequest, UpdatePriceListItemRequest, UpdatePriceListRequest, UpdateProductRequest, UpdateSkuRequest } from '../types';


export interface CatalogPriceListItemsListParams {
  listId: string;
  skuId?: string;
  status?: string;
  page?: number;
  pageSize?: number;
}

export interface CatalogPriceListItemsUpdateParams {
  ifMatch: string;
}

export interface CatalogPriceListItemsDeleteParams {
  ifMatch: string;
}

export class CatalogPriceListItemsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Catalog price list items list. */
  async list(params: CatalogPriceListItemsListParams, requestOptions?: ApiRequestOptions): Promise<{ items: PriceListItem[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'list_id', value: params.listId, style: 'form', explode: true, allowReserved: false },
      { name: 'sku_id', value: params.skuId, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: PriceListItem[]; pageInfo: PageInfo; }>(appendQueryString(backendApiPath(`/catalog/price_list_items`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Catalog price list items create. */
  async create(body: CreatePriceListItemRequest, requestOptions?: ApiRequestOptions): Promise<PriceListItem> {
    return this.client.request<PriceListItem>(backendApiPath(`/catalog/price_list_items`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Catalog price list items update. */
  async update(itemId: string, body: UpdatePriceListItemRequest, params: CatalogPriceListItemsUpdateParams, requestOptions?: ApiRequestOptions): Promise<PriceListItem> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<PriceListItem>(backendApiPath(`/catalog/price_list_items/${serializePathParameter(itemId, { name: 'itemId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Catalog price list items delete. */
  async delete(itemId: string, params: CatalogPriceListItemsDeleteParams, requestOptions?: ApiRequestOptions): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<void>(backendApiPath(`/catalog/price_list_items/${serializePathParameter(itemId, { name: 'itemId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any, ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}) });
  }
}

export interface CatalogAttributeValuesListParams {
  attributeId: string;
  status?: string;
  page?: number;
  pageSize?: number;
}

export interface CatalogAttributeValuesUpdateParams {
  ifMatch: string;
}

export interface CatalogAttributeValuesDeleteParams {
  ifMatch: string;
}

export class CatalogAttributeValuesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Catalog attribute values list. */
  async list(params: CatalogAttributeValuesListParams, requestOptions?: ApiRequestOptions): Promise<{ items: AttributeValue[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'attribute_id', value: params.attributeId, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: AttributeValue[]; pageInfo: PageInfo; }>(appendQueryString(backendApiPath(`/catalog/attribute_values`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Catalog attribute values create. */
  async create(body: CreateAttributeValueRequest, requestOptions?: ApiRequestOptions): Promise<AttributeValue> {
    return this.client.request<AttributeValue>(backendApiPath(`/catalog/attribute_values`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Catalog attribute values update. */
  async update(valueId: string, params: CatalogAttributeValuesUpdateParams, body?: UpdateAttributeValueRequest, requestOptions?: ApiRequestOptions): Promise<AttributeValue> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<AttributeValue>(backendApiPath(`/catalog/attribute_values/${serializePathParameter(valueId, { name: 'valueId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, ...(body !== undefined ? { body, contentType: 'application/json' } : {}), ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Catalog attribute values delete. */
  async delete(valueId: string, params: CatalogAttributeValuesDeleteParams, requestOptions?: ApiRequestOptions): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<void>(backendApiPath(`/catalog/attribute_values/${serializePathParameter(valueId, { name: 'valueId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any, ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}) });
  }
}

export interface CatalogMediaListParams {
  ownerType?: string;
  ownerId?: string;
  mediaRole?: string;
  status?: string;
  page?: number;
  pageSize?: number;
}

export interface CatalogMediaUpdateParams {
  ifMatch: string;
}

export interface CatalogMediaDeleteParams {
  ifMatch: string;
}

export class CatalogMediaApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Catalog media list. */
  async list(params?: CatalogMediaListParams, requestOptions?: ApiRequestOptions): Promise<{ items: Media[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'owner_type', value: params?.ownerType, style: 'form', explode: true, allowReserved: false },
      { name: 'owner_id', value: params?.ownerId, style: 'form', explode: true, allowReserved: false },
      { name: 'media_role', value: params?.mediaRole, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: Media[]; pageInfo: PageInfo; }>(appendQueryString(backendApiPath(`/catalog/media`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Catalog media create. */
  async create(body: CreateMediaRequest, requestOptions?: ApiRequestOptions): Promise<Media> {
    return this.client.request<Media>(backendApiPath(`/catalog/media`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Catalog media update. */
  async update(mediaId: string, body: UpdateMediaRequest, params: CatalogMediaUpdateParams, requestOptions?: ApiRequestOptions): Promise<Media> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<Media>(backendApiPath(`/catalog/media/${serializePathParameter(mediaId, { name: 'mediaId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Catalog media delete. */
  async delete(mediaId: string, params: CatalogMediaDeleteParams, requestOptions?: ApiRequestOptions): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<void>(backendApiPath(`/catalog/media/${serializePathParameter(mediaId, { name: 'mediaId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any, ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}) });
  }
}

export interface CatalogPriceListsListParams {
  currencyCode?: string;
  marketCode?: string;
  customerSegment?: string;
  status?: string;
  page?: number;
  pageSize?: number;
}

export interface CatalogPriceListsUpdateParams {
  ifMatch: string;
}

export class CatalogPriceListsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Catalog price Lists list. */
  async list(params?: CatalogPriceListsListParams, requestOptions?: ApiRequestOptions): Promise<{ items: PriceList[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'currency_code', value: params?.currencyCode, style: 'form', explode: true, allowReserved: false },
      { name: 'market_code', value: params?.marketCode, style: 'form', explode: true, allowReserved: false },
      { name: 'customer_segment', value: params?.customerSegment, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: PriceList[]; pageInfo: PageInfo; }>(appendQueryString(backendApiPath(`/catalog/price_lists`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Catalog price Lists create. */
  async create(body: CreatePriceListRequest, requestOptions?: ApiRequestOptions): Promise<PriceList> {
    return this.client.request<PriceList>(backendApiPath(`/catalog/price_lists`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Catalog price Lists update. */
  async update(priceListId: string, params: CatalogPriceListsUpdateParams, body?: UpdatePriceListRequest, requestOptions?: ApiRequestOptions): Promise<PriceList> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<PriceList>(backendApiPath(`/catalog/price_lists/${serializePathParameter(priceListId, { name: 'priceListId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, ...(body !== undefined ? { body, contentType: 'application/json' } : {}), ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Catalog price lists retrieve. */
  async retrieve(priceListId: string, requestOptions?: ApiRequestOptions): Promise<PriceList> {
    return this.client.request<PriceList>(backendApiPath(`/catalog/price_lists/${serializePathParameter(priceListId, { name: 'priceListId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface CatalogCategoryAttributesListParams {
  categoryId?: string;
  attributeId?: string;
  status?: string;
  page?: number;
  pageSize?: number;
}

export interface CatalogCategoryAttributesUpdateParams {
  ifMatch: string;
}

export interface CatalogCategoryAttributesDeleteParams {
  ifMatch: string;
}

export class CatalogCategoryAttributesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Catalog category Attributes list. */
  async list(params?: CatalogCategoryAttributesListParams, requestOptions?: ApiRequestOptions): Promise<{ items: CategoryAttribute[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'category_id', value: params?.categoryId, style: 'form', explode: true, allowReserved: false },
      { name: 'attribute_id', value: params?.attributeId, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: CategoryAttribute[]; pageInfo: PageInfo; }>(appendQueryString(backendApiPath(`/catalog/category_attributes`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Catalog category Attributes create. */
  async create(body: CreateCategoryAttributeRequest, requestOptions?: ApiRequestOptions): Promise<CategoryAttribute> {
    return this.client.request<CategoryAttribute>(backendApiPath(`/catalog/category_attributes`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Catalog category Attributes update. */
  async update(bindingId: string, params: CatalogCategoryAttributesUpdateParams, body?: UpdateCategoryAttributeRequest, requestOptions?: ApiRequestOptions): Promise<CategoryAttribute> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<CategoryAttribute>(backendApiPath(`/catalog/category_attributes/${serializePathParameter(bindingId, { name: 'bindingId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, ...(body !== undefined ? { body, contentType: 'application/json' } : {}), ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Catalog category Attributes delete. */
  async delete(bindingId: string, params: CatalogCategoryAttributesDeleteParams, requestOptions?: ApiRequestOptions): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<void>(backendApiPath(`/catalog/category_attributes/${serializePathParameter(bindingId, { name: 'bindingId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any, ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}) });
  }
}

export interface CatalogAttributesManagementListParams {
  status?: string;
  page?: number;
  pageSize?: number;
}

export class CatalogAttributesManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Catalog attributes management list. */
  async list(params?: CatalogAttributesManagementListParams, requestOptions?: ApiRequestOptions): Promise<{ items: Attribute[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: Attribute[]; pageInfo: PageInfo; }>(appendQueryString(backendApiPath(`/catalog/attributes`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Catalog attributes management retrieve. */
  async retrieve(attributeId: string, requestOptions?: ApiRequestOptions): Promise<Attribute> {
    return this.client.request<Attribute>(backendApiPath(`/catalog/attributes/${serializePathParameter(attributeId, { name: 'attributeId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface CatalogAttributesUpdateParams {
  ifMatch: string;
}

export class CatalogAttributesApi {
  private client: HttpClient;
  public readonly management: CatalogAttributesManagementApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new CatalogAttributesManagementApi(client);
  }


/** Catalog attributes create. */
  async create(body: CreateAttributeRequest, requestOptions?: ApiRequestOptions): Promise<Attribute> {
    return this.client.request<Attribute>(backendApiPath(`/catalog/attributes`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Catalog attributes update. */
  async update(attributeId: string, params: CatalogAttributesUpdateParams, body?: UpdateAttributeRequest, requestOptions?: ApiRequestOptions): Promise<Attribute> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<Attribute>(backendApiPath(`/catalog/attributes/${serializePathParameter(attributeId, { name: 'attributeId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, ...(body !== undefined ? { body, contentType: 'application/json' } : {}), ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }
}

export interface CatalogSkusListParams {
  productId?: string;
  attributeValueId?: string;
  status?: string;
  page?: number;
  pageSize?: number;
}

export interface CatalogSkusUpdateParams {
  ifMatch: string;
}

export interface CatalogSkusDeleteParams {
  ifMatch: string;
}

export class CatalogSkusApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Catalog skus list. */
  async list(params?: CatalogSkusListParams, requestOptions?: ApiRequestOptions): Promise<{ items: Sku[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'product_id', value: params?.productId, style: 'form', explode: true, allowReserved: false },
      { name: 'attribute_value_id', value: params?.attributeValueId, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: Sku[]; pageInfo: PageInfo; }>(appendQueryString(backendApiPath(`/catalog/skus`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Catalog skus create. */
  async create(body: CreateSkuRequest, requestOptions?: ApiRequestOptions): Promise<Sku> {
    return this.client.request<Sku>(backendApiPath(`/catalog/skus`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Catalog skus update. */
  async update(skuId: string, params: CatalogSkusUpdateParams, body?: UpdateSkuRequest, requestOptions?: ApiRequestOptions): Promise<Sku> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<Sku>(backendApiPath(`/catalog/skus/${serializePathParameter(skuId, { name: 'skuId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, ...(body !== undefined ? { body, contentType: 'application/json' } : {}), ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Catalog skus delete. */
  async delete(skuId: string, params: CatalogSkusDeleteParams, requestOptions?: ApiRequestOptions): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<void>(backendApiPath(`/catalog/skus/${serializePathParameter(skuId, { name: 'skuId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any, ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}) });
  }

/** Catalog skus retrieve. */
  async retrieve(skuId: string, requestOptions?: ApiRequestOptions): Promise<Sku> {
    return this.client.request<Sku>(backendApiPath(`/catalog/skus/${serializePathParameter(skuId, { name: 'skuId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface CatalogProductsAttributesUpdateParams {
  ifMatch: string;
}

export class CatalogProductsAttributesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Catalog product attributes replace. */
  async update(productId: string, body: ReplaceProductAttributesRequest, params: CatalogProductsAttributesUpdateParams, requestOptions?: ApiRequestOptions): Promise<ProductAttributes> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<ProductAttributes>(backendApiPath(`/catalog/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}/attributes`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PUT' as any, body, contentType: 'application/json', ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }
}

export interface CatalogProductsManagementListParams {
  q?: string;
  categoryId?: string;
  productType?: string;
  status?: string;
  page?: number;
  pageSize?: number;
  sort?: string;
}

export class CatalogProductsManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Catalog products management list. */
  async list(params?: CatalogProductsManagementListParams, requestOptions?: ApiRequestOptions): Promise<{ items: Product[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'category_id', value: params?.categoryId, style: 'form', explode: true, allowReserved: false },
      { name: 'product_type', value: params?.productType, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'sort', value: params?.sort, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: Product[]; pageInfo: PageInfo; }>(appendQueryString(backendApiPath(`/catalog/products`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Catalog products management retrieve. */
  async retrieve(productId: string, requestOptions?: ApiRequestOptions): Promise<Product> {
    return this.client.request<Product>(backendApiPath(`/catalog/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface CatalogProductsUpdateParams {
  ifMatch: string;
}

export interface CatalogProductsDeleteParams {
  ifMatch: string;
}

export interface CatalogProductsPublishParams {
  ifMatch: string;
}

export interface CatalogProductsArchiveParams {
  ifMatch: string;
}

export class CatalogProductsApi {
  private client: HttpClient;
  public readonly management: CatalogProductsManagementApi;
  public readonly attributes: CatalogProductsAttributesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new CatalogProductsManagementApi(client);
    this.attributes = new CatalogProductsAttributesApi(client);
  }


/** Catalog products create. */
  async create(body: CreateProductRequest, requestOptions?: ApiRequestOptions): Promise<Product> {
    return this.client.request<Product>(backendApiPath(`/catalog/products`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Catalog products update. */
  async update(productId: string, params: CatalogProductsUpdateParams, body?: UpdateProductRequest, requestOptions?: ApiRequestOptions): Promise<Product> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<Product>(backendApiPath(`/catalog/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, ...(body !== undefined ? { body, contentType: 'application/json' } : {}), ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Catalog products delete. */
  async delete(productId: string, params: CatalogProductsDeleteParams, requestOptions?: ApiRequestOptions): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<void>(backendApiPath(`/catalog/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any, ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}) });
  }

/** Catalog spus publish. */
  async publish(productId: string, params: CatalogProductsPublishParams, requestOptions?: ApiRequestOptions): Promise<Product> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<Product>(backendApiPath(`/catalog/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}/publish`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Catalog spus archive. */
  async archive(productId: string, params: CatalogProductsArchiveParams, requestOptions?: ApiRequestOptions): Promise<Product> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<Product>(backendApiPath(`/catalog/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}/archive`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }
}

export interface CatalogCategoriesManagementListParams {
  parentId?: string;
  status?: string;
  page?: number;
  pageSize?: number;
}

export class CatalogCategoriesManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Catalog categories management list. */
  async list(params?: CatalogCategoriesManagementListParams, requestOptions?: ApiRequestOptions): Promise<{ items: Category[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'parent_id', value: params?.parentId, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: Category[]; pageInfo: PageInfo; }>(appendQueryString(backendApiPath(`/catalog/categories`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Catalog categories management retrieve. */
  async retrieve(categoryId: string, requestOptions?: ApiRequestOptions): Promise<Category> {
    return this.client.request<Category>(backendApiPath(`/catalog/categories/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface CatalogCategoriesUpdateParams {
  ifMatch: string;
}

export interface CatalogCategoriesDeleteParams {
  ifMatch: string;
}

export class CatalogCategoriesApi {
  private client: HttpClient;
  public readonly management: CatalogCategoriesManagementApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new CatalogCategoriesManagementApi(client);
  }


/** Catalog categories create. */
  async create(body: CreateCategoryRequest, requestOptions?: ApiRequestOptions): Promise<Category> {
    return this.client.request<Category>(backendApiPath(`/catalog/categories`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Catalog categories update. */
  async update(categoryId: string, params: CatalogCategoriesUpdateParams, body?: UpdateCategoryRequest, requestOptions?: ApiRequestOptions): Promise<Category> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<Category>(backendApiPath(`/catalog/categories/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, ...(body !== undefined ? { body, contentType: 'application/json' } : {}), ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Catalog categories delete. */
  async delete(categoryId: string, params: CatalogCategoriesDeleteParams, requestOptions?: ApiRequestOptions): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'If-Match': { value: params.ifMatch, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<void>(backendApiPath(`/catalog/categories/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any, ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}) });
  }
}

export class CatalogApi {
  public readonly categories: CatalogCategoriesApi;
  public readonly products: CatalogProductsApi;
  public readonly skus: CatalogSkusApi;
  public readonly attributes: CatalogAttributesApi;
  public readonly categoryAttributes: CatalogCategoryAttributesApi;
  public readonly priceLists: CatalogPriceListsApi;
  public readonly media: CatalogMediaApi;
  public readonly attributeValues: CatalogAttributeValuesApi;
  public readonly priceListItems: CatalogPriceListItemsApi;

  constructor(client: HttpClient) {
    this.categories = new CatalogCategoriesApi(client);
    this.products = new CatalogProductsApi(client);
    this.skus = new CatalogSkusApi(client);
    this.attributes = new CatalogAttributesApi(client);
    this.categoryAttributes = new CatalogCategoryAttributesApi(client);
    this.priceLists = new CatalogPriceListsApi(client);
    this.media = new CatalogMediaApi(client);
    this.attributeValues = new CatalogAttributeValuesApi(client);
    this.priceListItems = new CatalogPriceListItemsApi(client);
  }

}

export function createCatalogApi(client: HttpClient): CatalogApi {
  return new CatalogApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
function buildRequestHeaders(
  headers: Record<string, HeaderParameterSpec | undefined>,
  cookies: Record<string, HeaderParameterSpec | undefined> = {},
): Record<string, string> | undefined {
  const requestHeaders: Record<string, string> = {};

  for (const [name, parameter] of Object.entries(headers)) {
    const serialized = serializeParameterValue(parameter);
    if (serialized !== undefined) {
      requestHeaders[name] = serialized;
    }
  }

  const cookieHeader = buildCookieHeader(cookies);
  if (cookieHeader) {
    requestHeaders.Cookie = requestHeaders.Cookie
      ? `${requestHeaders.Cookie}; ${cookieHeader}`
      : cookieHeader;
  }

  return Object.keys(requestHeaders).length > 0 ? requestHeaders : undefined;
}

interface HeaderParameterSpec {
  value: unknown;
  style: string;
  explode: boolean;
  contentType?: string;
}

function buildCookieHeader(cookies: Record<string, HeaderParameterSpec | undefined>): string | undefined {
  const pairs: string[] = [];
  for (const [name, parameter] of Object.entries(cookies)) {
    const serialized = serializeParameterValue(parameter);
    if (serialized !== undefined) {
      pairs.push(`${encodeURIComponent(name)}=${encodeURIComponent(serialized)}`);
    }
  }
  return pairs.length > 0 ? pairs.join('; ') : undefined;
}

function serializeParameterValue(parameter: HeaderParameterSpec | undefined): string | undefined {
  const value = parameter?.value;
  if (value === undefined || value === null) {
    return undefined;
  }
  if (parameter?.contentType) {
    return JSON.stringify(value);
  }
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (Array.isArray(value)) {
    return value.map((item) => serializeHeaderPrimitive(item)).join(',');
  }
  if (typeof value === 'object' && value !== null) {
    return serializeHeaderObject(value as Record<string, unknown>, parameter?.explode === true);
  }
  return serializeHeaderPrimitive(value);
}

function serializeHeaderObject(value: Record<string, unknown>, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (explode) {
    return entries.map(([key, entryValue]) => `${key}=${serializeHeaderPrimitive(entryValue)}`).join(',');
  }
  return entries.flatMap(([key, entryValue]) => [key, serializeHeaderPrimitive(entryValue)]).join(',');
}

function serializeHeaderPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  return String(value);
}
