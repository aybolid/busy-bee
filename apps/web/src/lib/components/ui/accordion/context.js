import { createContext } from "svelte";

/**
 * @typedef {Object} AccordionContext
 * @property {string[]} openIds
 */

export const [getAccordionContext, setAccordionContext] =
    /** @type {typeof createContext<AccordionContext>} */ (createContext)();
