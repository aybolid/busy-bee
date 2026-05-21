import { createContext } from "svelte";

/**
 * @typedef {Object} DialogContext
 * @property {string} dialogId
 * @property {string} labelId
 * @property {string} descriptionId
 */

export const [getDialogContext, setDialogContext] =
    /** @type {typeof createContext<DialogContext>} */ (createContext)();
