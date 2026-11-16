import { useEffect, useState } from "react";
import { Minus, Plus, ShoppingBag, Store, Trash2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { cn } from "@sdkwork/ui-mobile-react";

import { formatCny } from "../money";
import type { CartItem } from "../types";
import { useCartStore } from "../store/useCartStore";

/** Host-overridable routes for the cart surface. */
export interface ShoppingCartPageProps {
  checkoutPath?: string;
  productDetailPath?: string;
}

const ACCENT = "var(--shopping-accent,#FA5151)";

/** Groups lines by shop so a multi-shop cart reads like a checkout summary. */
function groupCartItems(items: readonly CartItem[]) {
  const groups = new Map<string, { items: CartItem[]; shopId: string; shopName?: string }>();
  for (const item of items) {
    const shopId = item.product.shopId || "default";
    const group = groups.get(shopId) ?? { items: [], shopId, shopName: item.shopName };
    group.items.push(item);
    groups.set(shopId, group);
  }
  return Array.from(groups.values());
}

/**
 * Shopping cart surface.
 *
 * The page owns cart presentation and selection state; every data mutation goes
 * through the injected {@link ShopMobileCartRuntime} port so a host can bind its
 * own cart API/service (mall binds its commerce cart facade). Empty/loading
 * states and the checkout handoff are part of the surface so every host gets the
 * same behaviour.
 */
export function ShoppingCartPage({
  checkoutPath = "/checkout",
  productDetailPath = "/product/:productId",
}: ShoppingCartPageProps = {}) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const items = useCartStore((state) => state.items);
  const loading = useCartStore((state) => state.loading);
  const loadCart = useCartStore((state) => state.loadCart);
  const updateQuantity = useCartStore((state) => state.updateQuantity);
  const toggleItemCheck = useCartStore((state) => state.toggleItemCheck);
  const toggleAllCheck = useCartStore((state) => state.toggleAllCheck);
  const removeFromCart = useCartStore((state) => state.removeFromCart);
  const [busyId, setBusyId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let active = true;
    loadCart().catch((cause: unknown) => {
      if (active) {
        setError(cause instanceof Error ? cause.message : t("shopping.cart_load_failed", "购物车加载失败"));
      }
    });
    return () => {
      active = false;
    };
  }, [loadCart, t]);

  const checkedItems = items.filter((item) => item.checked);
  const groups = groupCartItems(items);
  const allChecked = items.length > 0 && checkedItems.length === items.length;
  const totalAmount = checkedItems.reduce((total, item) => {
    const price = Number.parseFloat(item.sku?.price ?? item.product.price ?? "0");
    return total + (Number.isFinite(price) ? price : 0) * item.quantity;
  }, 0);

  async function run(itemId: string, action: () => Promise<void>): Promise<void> {
    setBusyId(itemId);
    setError(null);
    try {
      await action();
      await loadCart();
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : t("shopping.cart_update_failed", "操作失败"));
    } finally {
      setBusyId(null);
    }
  }

  function openProduct(productId: string): void {
    navigate(productDetailPath.replace(":productId", encodeURIComponent(productId)));
  }

  function handleCheckout(): void {
    const ids = checkedItems.map((item) => item.id);
    if (ids.length === 0) {
      return;
    }
    // The host checkout reads the selected lines from `items`, so the cart owns
    // the selection and checkout never falls back to "everything".
    navigate(`${checkoutPath}?items=${encodeURIComponent(ids.join(","))}`);
  }

  return (
    <div className="flex min-h-full flex-col bg-bg-color">
      <header className="flex items-baseline justify-between px-4 pb-2 pt-safe">
        <h1 className="text-[20px] font-semibold text-text-main">
          {t("shopping.cart_title", "购物车")}
        </h1>
        <span className="text-[12px] text-text-sub">
          {t("shopping.cart_count", "共 {{count}} 件", { count: items.length })}
        </span>
      </header>

      {loading && items.length === 0 ? (
        <div className="flex flex-1 items-center justify-center py-20 text-[13px] text-text-sub">
          {t("shopping.cart_loading", "加载购物车...")}
        </div>
      ) : error ? (
        <div className="flex flex-1 flex-col items-center justify-center gap-3 px-8 py-16 text-center">
          <p className="text-[13px] text-text-sub">{error}</p>
          <button
            className="rounded-full px-4 py-1.5 text-[13px] text-white"
            onClick={() => void loadCart().catch(() => undefined)}
            style={{ backgroundColor: ACCENT }}
            type="button"
          >
            {t("shopping.retry", "重试")}
          </button>
        </div>
      ) : items.length === 0 ? (
        <div className="flex flex-1 flex-col items-center justify-center gap-3 px-8 py-20 text-center">
          <div className="flex h-16 w-16 items-center justify-center rounded-full bg-fill">
            <ShoppingBag className="h-7 w-7 text-text-sub" />
          </div>
          <p className="text-[15px] font-medium text-text-main">
            {t("shopping.cart_empty_title", "购物车还是空的")}
          </p>
          <p className="text-[12px] text-text-sub">
            {t("shopping.cart_empty_hint", "去逛逛，把心仪的商品加进来")}
          </p>
          <button
            className="mt-2 rounded-full px-5 py-2 text-[14px] font-medium text-white"
            onClick={() => navigate("/")}
            style={{ backgroundColor: ACCENT }}
            type="button"
          >
            {t("shopping.cart_empty_action", "去逛逛")}
          </button>
        </div>
      ) : (
        <>
          <div className="flex flex-1 flex-col gap-2 px-3 pb-28">
            {groups.map((group) => (
              <div className="flex flex-col gap-2" key={group.shopId}>
                {group.shopName ? (
                  <div className="flex items-center gap-1.5 px-1 pt-1 text-[12px] text-text-sub">
                    <Store className="h-3.5 w-3.5" />
                    <span className="line-clamp-1">{group.shopName}</span>
                  </div>
                ) : null}
                {group.items.map((item) => {
              const busy = busyId === item.id;
              return (
                <div className="bg-chat-other-bg rounded-xl p-3 flex gap-3" key={item.id}>
                  <button
                    aria-checked={item.checked}
                    aria-label={t("shopping.select_item", "选择商品")}
                    className={cn(
                      "mt-1 h-5 w-5 shrink-0 rounded-full border flex items-center justify-center",
                      item.checked ? "border-transparent" : "border-text-sub/40 bg-transparent",
                    )}
                    disabled={busy}
                    onClick={() => void run(item.id, () => toggleItemCheck(item.id, !item.checked))}
                    role="checkbox"
                    style={item.checked ? { backgroundColor: ACCENT } : undefined}
                    type="button"
                  >
                    {item.checked ? <span className="h-1.5 w-1.5 rounded-full bg-white" /> : null}
                  </button>

                  <button
                    aria-label={t("shopping.open_product", "查看商品")}
                    className="h-20 w-20 shrink-0 overflow-hidden rounded-lg bg-fill"
                    onClick={() => openProduct(item.productId)}
                    type="button"
                  >
                    {item.product.image ? (
                      <img
                        alt={item.product.title}
                        className="h-full w-full object-cover"
                        src={item.product.image}
                      />
                    ) : (
                      <span className="flex h-full w-full items-center justify-center">
                        <ShoppingBag className="h-7 w-7 text-text-sub/40" />
                      </span>
                    )}
                  </button>

                  <div className="flex min-w-0 flex-1 flex-col py-0.5">
                    <button
                      className="line-clamp-2 text-left text-[14px] leading-tight text-text-main"
                      onClick={() => openProduct(item.productId)}
                      type="button"
                    >
                      {item.product.title}
                    </button>
                    {item.sku ? (
                      <span className="mt-1 w-fit rounded bg-fill px-1.5 py-0.5 text-[11px] text-text-sub">
                        {item.sku.specValues
                          ? Object.values(item.sku.specValues).join(" / ")
                          : item.sku.price}
                      </span>
                    ) : null}
                    <div className="mt-auto flex items-center justify-between">
                      <span className="text-[16px] font-bold" style={{ color: ACCENT }}>
                        {formatCny(item.sku?.price ?? item.product.price, "zh-CN") ?? "--"}
                      </span>
                      <div className="flex items-center overflow-hidden rounded-md border border-border-color">
                        <button
                          aria-label={t("shopping.decrease", "减少数量")}
                          className="flex h-6 w-7 items-center justify-center text-[14px] text-text-main active:bg-fill disabled:opacity-40"
                          disabled={busy || item.quantity <= 1}
                          onClick={() => void run(item.id, () => updateQuantity(item.id, Math.max(1, item.quantity - 1)))}
                          type="button"
                        >
                          <Minus className="h-3 w-3" />
                        </button>
                        <span className="flex h-6 w-8 items-center justify-center border-x border-border-color text-[13px] text-text-main">
                          {item.quantity}
                        </span>
                        <button
                          aria-label={t("shopping.increase", "增加数量")}
                          className="flex h-6 w-7 items-center justify-center text-[14px] text-text-main active:bg-fill disabled:opacity-40"
                          disabled={busy}
                          onClick={() => void run(item.id, () => updateQuantity(item.id, item.quantity + 1))}
                          type="button"
                        >
                          <Plus className="h-3 w-3" />
                        </button>
                      </div>
                    </div>
                  </div>

                  <button
                    aria-label={t("shopping.remove", "删除商品")}
                    className="self-start p-1.5 text-text-sub/50 active:text-text-main"
                    disabled={busy}
                    onClick={() => void run(item.id, () => removeFromCart([item.id]))}
                    type="button"
                  >
                    <Trash2 className="h-4 w-4" />
                  </button>
                </div>
              );
                })}
              </div>
            ))}
          </div>

          <div className="sticky bottom-0 z-10 flex items-center gap-3 border-t border-border-color bg-chat-other-bg px-3 py-2.5">
            <button
              aria-checked={allChecked}
              aria-label={t("shopping.select_all", "全选")}
              className="flex items-center gap-1.5 text-[12px] text-text-sub"
              onClick={() => void run("all", () => toggleAllCheck(!allChecked))}
              role="checkbox"
              type="button"
            >
              <span
                className={cn(
                  "flex h-4 w-4 items-center justify-center rounded-full border",
                  allChecked ? "border-transparent" : "border-text-sub/40",
                )}
                style={allChecked ? { backgroundColor: ACCENT } : undefined}
              >
                {allChecked ? <span className="h-1.5 w-1.5 rounded-full bg-white" /> : null}
              </span>
              {t("shopping.select_all", "全选")}
            </button>
            <div className="ml-auto flex items-baseline gap-1">
              <span className="text-[12px] text-text-sub">{t("shopping.total", "合计")}</span>
              <span className="text-[18px] font-bold" style={{ color: ACCENT }}>
                {formatCny(String(totalAmount), "zh-CN") ?? "--"}
              </span>
            </div>
            <button
              className="rounded-full px-5 py-2 text-[14px] font-medium text-white disabled:opacity-50"
              disabled={checkedItems.length === 0}
              onClick={handleCheckout}
              style={{ backgroundColor: ACCENT }}
              type="button"
            >
              {t("shopping.checkout", "去结算")}
              {checkedItems.length > 0 ? ` (${checkedItems.length})` : ""}
            </button>
          </div>
        </>
      )}
    </div>
  );
}
