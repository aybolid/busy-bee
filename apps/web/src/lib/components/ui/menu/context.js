import { createContext } from "svelte";

/**
 * @typedef {Object} MenuContext
 * @property {string} menuId
 */

export const [getMenuContext, setMenuContext] = /** @type {typeof createContext<MenuContext>} */ (
    createContext
)();
