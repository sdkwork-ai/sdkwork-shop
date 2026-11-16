import type { CartItem, Product, ProductSKU } from "../types";
import { ShoppingCapabilityUnavailableError } from "./ShoppingCapabilityUnavailableError";

/**
 * Host-injected cart capability.
 *
 * The shop surface owns the cart UI/state, but the cart data plane belongs to
 * the host's composed commerce service (the shop app-api does not own carts).
 * A host wires this port once at bootstrap; methods the host does not support
 * fail closed with {@link ShoppingCapabilityUnavailableError} instead of
 * pretending the cart is empty.
 */
export interface ShopMobileCartRuntime {
  /** Reads the current cart, including the host's selection state. */
  getCart(): Promise<CartItem[]>;
  /** Persists a line quantity. */
  updateQuantity(cartItemId: string, quantity: number): Promise<void>;
  /** Persists the per-line selection checkbox. */
  toggleCheck(cartItemId: string, checked: boolean): Promise<void>;
  /** Persists the select-all checkbox across the cart. */
  toggleAllCheck(checked: boolean): Promise<void>;
  /** Removes the given cart lines. */
  removeFromCart(cartItemIds: string[]): Promise<void>;
  /** Empties the cart. */
  clearCart(): Promise<void>;
  /** Optional: adds a product (with an optional SKU) to the cart. */
  addToCart?(
    product: Product,
    quantity: number,
    sku?: ProductSKU,
    selectedSpecs?: Record<string, string>,
  ): Promise<void>;
}

let runtime: ShopMobileCartRuntime | null = null;

/** Wires the host cart capability for the mounted shopping surfaces. */
export function configureShopMobileCartRuntime(nextRuntime: ShopMobileCartRuntime | null): void {
  runtime = nextRuntime;
}

/** Returns the injected runtime, or `null` when the host did not compose one. */
export function getShopMobileCartRuntime(): ShopMobileCartRuntime | null {
  return runtime;
}

function requireRuntime(): ShopMobileCartRuntime {
  if (!runtime) {
    throw new ShoppingCapabilityUnavailableError();
  }
  return runtime;
}

export const CartService = {
  async getCart(): Promise<CartItem[]> {
    return requireRuntime().getCart();
  },

  async addToCart(
    product: Product,
    quantity = 1,
    sku?: ProductSKU,
    selectedSpecs?: Record<string, string>,
  ): Promise<void> {
    const active = requireRuntime();
    if (!active.addToCart) {
      throw new ShoppingCapabilityUnavailableError();
    }
    await active.addToCart(product, quantity, sku, selectedSpecs);
  },

  async updateQuantity(cartItemId: string, quantity: number): Promise<void> {
    await requireRuntime().updateQuantity(cartItemId, quantity);
  },

  async toggleCheck(cartItemId: string, checked: boolean): Promise<void> {
    await requireRuntime().toggleCheck(cartItemId, checked);
  },

  async toggleAllCheck(checked: boolean): Promise<void> {
    await requireRuntime().toggleAllCheck(checked);
  },

  async removeFromCart(cartItemIds: string[]): Promise<void> {
    await requireRuntime().removeFromCart(cartItemIds);
  },

  async clearCart(): Promise<void> {
    await requireRuntime().clearCart();
  },
};
