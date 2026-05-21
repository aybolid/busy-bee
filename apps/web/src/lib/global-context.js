import { createContext } from "svelte";

/**
 * @typedef {Object} GlobalContext
 * @property {import('ky').KyInstance} ky
 * @property {import('@tanstack/svelte-query').QueryClient} queryClient
 */

export const [getGlobalContext, setGlobalContext] =
    /** @type {typeof createContext<GlobalContext>} */ (createContext)();
