import { createContext } from "svelte";

/**
 * @typedef {Object} AccordionItemContext
 * @property {string} id
 */

export const [getAccordionItemContext, setAccordionItemContext] =
    /** @type {typeof createContext<AccordionItemContext>} */ (createContext)();
