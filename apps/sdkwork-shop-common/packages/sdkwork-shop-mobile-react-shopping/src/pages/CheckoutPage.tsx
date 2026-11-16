import { useCallback, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useSearchParams } from "react-router";

import { PageLayout } from "@sdkwork/ui-mobile-react";

import { formatCny } from "../money";
import {
  getShopMobileCheckoutRuntime,
  type ShopCheckoutAddress,
  type ShopCheckoutCoupon,
  type ShopCheckoutInvoiceForm,
  type ShopCheckoutItem,
  type ShopCheckoutPaymentMethod,
  type ShopCheckoutQuote,
} from "../services/CheckoutService";

/** Delivery options offered by the confirmation screen. */
const DELIVERY_METHODS: ReadonlyArray<{ code: string; label: string; hint: string }> = [
  { code: "standard", label: "普通快递", hint: "预计 2-4 天送达" },
  { code: "express", label: "加急配送", hint: "预计次日送达" },
];

function money(value: string | undefined, fallback = "--"): string {
  return value ? (formatCny(value, "zh-CN") ?? fallback) : fallback;
}

/**
 * Confirm-order screen (the purchase funnel's last host-owned step).
 *
 * Owns address / delivery / payment / coupon / deduction / gift-card / invoice /
 * remark selection, the quote summary and the submit handoff. Every read and
 * write goes through the injected {@link ShopCheckoutRuntime}; the host supplies
 * its commerce checkout facade.
 */
export function CheckoutPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const cartItemIds = useMemo(
    () => (searchParams.get("items") ?? "").split(",").filter(Boolean),
    [searchParams],
  );
  const runtime = getShopMobileCheckoutRuntime();

  const [items, setItems] = useState<readonly ShopCheckoutItem[]>([]);
  const [addresses, setAddresses] = useState<readonly ShopCheckoutAddress[]>([]);
  const [paymentMethods, setPaymentMethods] = useState<readonly ShopCheckoutPaymentMethod[]>([]);
  const [coupons, setCoupons] = useState<readonly ShopCheckoutCoupon[]>([]);
  const [quote, setQuote] = useState<ShopCheckoutQuote | null>(null);
  const [selectedAddressId, setSelectedAddressId] = useState("");
  const [selectedMethodCode, setSelectedMethodCode] = useState("");
  const [selectedCouponId, setSelectedCouponId] = useState("");
  const [deliveryMethod, setDeliveryMethod] = useState("standard");
  const [useWallet, setUseWallet] = useState(false);
  const [usePoints, setUsePoints] = useState(false);
  const [giftCardCode, setGiftCardCode] = useState("");
  const [needInvoice, setNeedInvoice] = useState(false);
  const [invoiceForm, setInvoiceForm] = useState<ShopCheckoutInvoiceForm>({ title: "", titleType: "personal" });
  const [buyerRemark, setBuyerRemark] = useState("");
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    if (!runtime) {
      setError(t("shopping.checkout_unavailable", "结算能力尚未接入"));
      setLoading(false);
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const [nextItems, nextAddresses, nextMethods, nextCoupons] = await Promise.all([
        runtime.loadItems(),
        runtime.loadAddresses(),
        runtime.loadPaymentMethods(),
        runtime.loadCoupons(),
      ]);
      setItems(nextItems);
      setAddresses(nextAddresses);
      setPaymentMethods(nextMethods);
      setCoupons(nextCoupons);
      setSelectedAddressId((current) =>
        current || nextAddresses.find((address) => address.isDefault)?.id || nextAddresses[0]?.id || "");
      setSelectedMethodCode((current) => current || nextMethods[0]?.code || "");
      setQuote(await runtime.createQuote(cartItemIds.length > 0 ? { cartItemIds } : {}));
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : t("shopping.checkout_load_failed", "结算信息加载失败"));
    } finally {
      setLoading(false);
    }
  }, [cartItemIds, runtime, t]);

  useEffect(() => {
    void load();
  }, [load]);

  async function submit(): Promise<void> {
    if (!runtime || submitting) {
      return;
    }
    setSubmitting(true);
    setError(null);
    try {
      const result = await runtime.submit({
        ...(selectedAddressId ? { addressId: selectedAddressId } : {}),
        ...(buyerRemark.trim() ? { buyerRemark: buyerRemark.trim() } : {}),
        ...(cartItemIds.length > 0 ? { cartItemIds } : {}),
        ...(selectedCouponId ? { couponId: selectedCouponId } : {}),
        deliveryMethod,
        ...(giftCardCode.trim() ? { giftCardCode: giftCardCode.trim() } : {}),
        ...(needInvoice ? { invoiceForm } : {}),
        needInvoice,
        ...(selectedMethodCode ? { paymentMethodCode: selectedMethodCode } : {}),
        usePoints,
        useWallet,
      });
      navigate(result.nextUrl);
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : t("shopping.checkout_submit_failed", "提交订单失败"));
      setSubmitting(false);
    }
  }

  return (
    <PageLayout title={t("shopping.checkout_title", "确认订单")}>
      <h1 className="sr-only">{t("shopping.checkout_title", "确认订单")}</h1>
      <div className="flex flex-1 flex-col gap-3 overflow-y-auto px-3 py-3 pb-28">
        {loading ? (
          <p className="py-10 text-center text-[13px] text-text-sub">
            {t("shopping.checkout_loading", "加载结算信息...")}
          </p>
        ) : null}
        {error ? (
          <div className="flex flex-col items-center gap-3 rounded-xl bg-chat-other-bg p-4 text-center">
            <p className="text-[13px] text-text-sub">{error}</p>
            <button
              className="rounded-full border border-border-color px-4 py-1.5 text-[13px] text-text-main active:bg-active-bg"
              onClick={() => void load()}
              type="button"
            >
              {t("shopping.retry", "重试")}
            </button>
          </div>
        ) : null}

        {!loading && !error ? (
          <>
            <section className="rounded-xl bg-chat-other-bg p-4">
              <h2 className="mb-3 text-[14px] font-semibold text-text-main">
                {t("shopping.checkout_address", "收货地址")}
              </h2>
              {addresses.length === 0 ? (
                <p className="text-[13px] text-text-sub">{t("shopping.checkout_no_address", "暂无收货地址，请先添加")}</p>
              ) : (
                <div className="flex flex-col gap-2">
                  {addresses.map((address) => (
                    <label className="flex cursor-pointer items-start gap-2" key={address.id}>
                      <input
                        checked={selectedAddressId === address.id}
                        name="checkout-address"
                        onChange={() => setSelectedAddressId(address.id)}
                        type="radio"
                      />
                      <span className="min-w-0 flex-1 text-[13px] text-text-main">
                        {address.receiverName} {address.receiverPhone}
                        {address.isDefault ? (
                          <em className="ml-1 rounded bg-primary-blue/10 px-1 text-[10px] not-italic text-primary-blue">
                            {t("shopping.default", "默认")}
                          </em>
                        ) : null}
                        <br />
                        <small className="text-[12px] text-text-sub">{address.addressLine}</small>
                      </span>
                    </label>
                  ))}
                </div>
              )}
            </section>

            <section className="rounded-xl bg-chat-other-bg p-4">
              <h2 className="mb-2 text-[14px] font-semibold text-text-main">
                {t("shopping.checkout_items", "商品清单")}
              </h2>
              {items.map((item) => (
                <div className="flex gap-2.5 py-2" key={item.id}>
                  <div className="h-14 w-14 shrink-0 overflow-hidden rounded-lg bg-fill">
                    {item.image ? (
                      <img alt={item.title} className="h-full w-full object-cover" src={item.image} />
                    ) : null}
                  </div>
                  <div className="min-w-0 flex-1">
                    <p className="line-clamp-2 text-[13px] text-text-main">{item.title}</p>
                    {item.skuName ? <p className="text-[11px] text-text-sub">{item.skuName}</p> : null}
                  </div>
                  <div className="text-right text-[13px] text-text-main">
                    <p>{money(item.price)}</p>
                    <p className="text-[11px] text-text-sub">x{item.quantity}</p>
                  </div>
                </div>
              ))}
            </section>

            <section className="rounded-xl bg-chat-other-bg p-4">
              <h2 className="mb-2 text-[14px] font-semibold text-text-main">
                {t("shopping.checkout_delivery", "配送方式")}
              </h2>
              {DELIVERY_METHODS.map((method) => (
                <label className="flex cursor-pointer items-start gap-2 py-1" key={method.code}>
                  <input
                    checked={deliveryMethod === method.code}
                    name="checkout-delivery"
                    onChange={() => setDeliveryMethod(method.code)}
                    type="radio"
                  />
                  <span className="text-[13px] text-text-main">
                    {t(`shopping.delivery_${method.code}`, method.label)}
                    <br />
                    <small className="text-[12px] text-text-sub">
                      {t(`shopping.delivery_${method.code}_hint`, method.hint)}
                    </small>
                  </span>
                </label>
              ))}
            </section>

            {paymentMethods.length > 0 ? (
              <section className="rounded-xl bg-chat-other-bg p-4">
                <h2 className="mb-2 text-[14px] font-semibold text-text-main">
                  {t("shopping.checkout_payment", "支付方式")}
                </h2>
                {paymentMethods.map((method) => (
                  <label className="flex cursor-pointer items-center gap-2 py-1" key={method.code}>
                    <input
                      checked={selectedMethodCode === method.code}
                      name="checkout-payment"
                      onChange={() => setSelectedMethodCode(method.code)}
                      type="radio"
                    />
                    <span className="text-[13px] text-text-main">{method.label}</span>
                  </label>
                ))}
              </section>
            ) : null}

            <section className="rounded-xl bg-chat-other-bg p-4">
              <h2 className="mb-2 text-[14px] font-semibold text-text-main">
                {t("shopping.checkout_coupon", "优惠券")}
              </h2>
              <label className="flex cursor-pointer items-center gap-2 py-1">
                <input
                  checked={!selectedCouponId}
                  name="checkout-coupon"
                  onChange={() => setSelectedCouponId("")}
                  type="radio"
                />
                <span className="text-[13px] text-text-main">{t("shopping.checkout_coupon_none", "不使用优惠券")}</span>
              </label>
              {coupons.map((coupon) => (
                <label className="flex cursor-pointer items-start gap-2 py-1" key={coupon.id}>
                  <input
                    checked={selectedCouponId === coupon.id}
                    name="checkout-coupon"
                    onChange={() => setSelectedCouponId(coupon.id)}
                    type="radio"
                  />
                  <span className="text-[13px] text-text-main">
                    {coupon.title}
                    {coupon.discount ? (
                      <em className="ml-1 not-italic text-primary-blue">-{money(coupon.discount)}</em>
                    ) : null}
                  </span>
                </label>
              ))}
            </section>

            <section className="rounded-xl bg-chat-other-bg p-4">
              <h2 className="mb-2 text-[14px] font-semibold text-text-main">
                {t("shopping.checkout_deduction", "抵扣")}
              </h2>
              <label className="flex items-center gap-2 py-1 text-[13px] text-text-main">
                <input checked={useWallet} onChange={(event) => setUseWallet(event.target.checked)} type="checkbox" />
                {t("shopping.use_wallet", "使用钱包余额")}
              </label>
              <label className="flex items-center gap-2 py-1 text-[13px] text-text-main">
                <input checked={usePoints} onChange={(event) => setUsePoints(event.target.checked)} type="checkbox" />
                {t("shopping.use_points", "使用积分")}
              </label>
            </section>

            <section className="rounded-xl bg-chat-other-bg p-4">
              <h2 className="mb-2 text-[14px] font-semibold text-text-main">
                {t("shopping.checkout_extra", "礼品卡与发票")}
              </h2>
              <input
                className="w-full rounded-lg bg-bg-color px-3 py-2 text-[13px] text-text-main outline-none"
                onChange={(event) => setGiftCardCode(event.target.value)}
                placeholder={t("shopping.gift_card_placeholder", "礼品卡/券码（选填）")}
                value={giftCardCode}
              />
              <label className="mt-2 flex items-center gap-2 text-[13px] text-text-main">
                <input checked={needInvoice} onChange={(event) => setNeedInvoice(event.target.checked)} type="checkbox" />
                {t("shopping.need_invoice", "需要发票")}
              </label>
              {needInvoice ? (
                <div className="mt-2 flex flex-col gap-2">
                  <input
                    className="w-full rounded-lg bg-bg-color px-3 py-2 text-[13px] text-text-main outline-none"
                    onChange={(event) => setInvoiceForm((previous) => ({ ...previous, title: event.target.value }))}
                    placeholder={t("shopping.invoice_title", "发票抬头")}
                    value={invoiceForm.title}
                  />
                  <input
                    className="w-full rounded-lg bg-bg-color px-3 py-2 text-[13px] text-text-main outline-none"
                    onChange={(event) => setInvoiceForm((previous) => ({ ...previous, taxNumber: event.target.value }))}
                    placeholder={t("shopping.invoice_tax", "税号（企业抬头）")}
                    value={invoiceForm.taxNumber ?? ""}
                  />
                </div>
              ) : null}
            </section>

            <section className="rounded-xl bg-chat-other-bg p-4">
              <h2 className="mb-2 text-[14px] font-semibold text-text-main">
                {t("shopping.checkout_remark", "买家留言")}
              </h2>
              <input
                className="w-full rounded-lg bg-bg-color px-3 py-2 text-[13px] text-text-main outline-none"
                onChange={(event) => setBuyerRemark(event.target.value)}
                placeholder={t("shopping.checkout_remark_placeholder", "选填，如：请轻放")}
                value={buyerRemark}
              />
            </section>

            <section className="rounded-xl bg-chat-other-bg p-4">
              <div className="flex items-center justify-between text-[13px] text-text-sub">
                <span>{t("shopping.checkout_original", "商品总额")}</span>
                <span>{money(quote?.originalAmount)}</span>
              </div>
              <div className="mt-1 flex items-center justify-between text-[13px] text-text-sub">
                <span>{t("shopping.checkout_discount", "优惠")}</span>
                <span>-{money(quote?.discountAmount)}</span>
              </div>
              <div className="mt-2 flex items-center justify-between border-t border-border-color pt-2 text-[14px] text-text-main">
                <span>{t("shopping.checkout_payable", "应付金额")}</span>
                <span className="text-[18px] font-bold text-primary-blue">{money(quote?.payableAmount)}</span>
              </div>
            </section>
          </>
        ) : null}
      </div>

      <div className="sticky bottom-0 z-10 flex items-center gap-3 border-t border-border-color bg-chat-other-bg px-3 py-2.5">
        <div className="flex items-baseline gap-1">
          <span className="text-[12px] text-text-sub">{t("shopping.checkout_total", "合计")}</span>
          <span className="text-[18px] font-bold text-primary-blue">{money(quote?.payableAmount)}</span>
        </div>
        <button
          className="ml-auto rounded-full bg-primary-blue px-6 py-2 text-[14px] font-medium text-white active:opacity-80 disabled:opacity-50"
          disabled={loading || submitting || Boolean(error) || !selectedAddressId}
          onClick={() => void submit()}
          type="button"
        >
          {submitting ? t("shopping.checkout_submitting", "提交中...") : t("shopping.checkout_submit", "提交订单")}
        </button>
      </div>
    </PageLayout>
  );
}
