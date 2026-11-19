/**
 * Host-injected checkout capability.
 *
 * The shop surface owns the confirm-order screen (address, delivery, payment
 * method, coupons, wallet/points deduction, gift card, invoice, remark, quote
 * and submit). The order/checkout data plane belongs to the host's commerce
 * service, so a host injects it once here; before injection the page fails
 * closed instead of rendering an empty form that cannot submit.
 */
export interface ShopCheckoutItem {
  id: string;
  title: string;
  image?: string;
  /** Major-unit price string, e.g. "129.00". */
  price: string;
  quantity: number;
  skuName?: string;
}

export interface ShopCheckoutAddress {
  id: string;
  receiverName: string;
  receiverPhone: string;
  addressLine: string;
  isDefault: boolean;
}

export interface ShopCheckoutPaymentMethod {
  code: string;
  label: string;
}

export interface ShopCheckoutCoupon {
  id: string;
  title: string;
  discount?: string;
  minSpend?: string;
}

export interface ShopCheckoutQuote {
  payableAmount?: string;
  originalAmount?: string;
  discountAmount?: string;
}

export interface ShopCheckoutInvoiceForm {
  email?: string;
  taxNumber?: string;
  title: string;
  titleType?: string;
}

export interface ShopCheckoutSubmitInput {
  addressId?: string;
  buyerRemark?: string;
  cartItemIds?: string[];
  couponId?: string;
  deliveryMethod?: string;
  giftCardCode?: string;
  invoiceForm?: ShopCheckoutInvoiceForm;
  needInvoice?: boolean;
  paymentMethodCode?: string;
  usePoints?: boolean;
  useWallet?: boolean;
}

export interface ShopCheckoutRuntime {
  loadAddresses(): Promise<readonly ShopCheckoutAddress[]>;
  loadCoupons(): Promise<readonly ShopCheckoutCoupon[]>;
  loadItems(): Promise<readonly ShopCheckoutItem[]>;
  loadPaymentMethods(): Promise<readonly ShopCheckoutPaymentMethod[]>;
  createQuote(input: {
    cartItemIds?: string[];
    couponId?: string;
    deliveryMethod?: string;
    usePoints?: boolean;
    useWallet?: boolean;
  }): Promise<ShopCheckoutQuote>;
  submit(input: ShopCheckoutSubmitInput): Promise<{ nextUrl: string; warnings?: readonly string[] }>;
}

let runtime: ShopCheckoutRuntime | null = null;

/** Wires the host checkout capability for the confirm-order page. */
export function configureShopMobileCheckoutRuntime(nextRuntime: ShopCheckoutRuntime | null): void {
  runtime = nextRuntime;
}

/** Returns the injected runtime, or `null` when the host did not compose one. */
export function getShopMobileCheckoutRuntime(): ShopCheckoutRuntime | null {
  return runtime;
}
