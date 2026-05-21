import { createContext } from "svelte";

/**
 * @typedef {Object} AlertDialogContext
 * @property {string} dialogId
 * @property {string} labelId
 * @property {string} descriptionId
 */

export const [getAlertDialogContext, setAlertDialogContext] =
    /** @type {typeof createContext<AlertDialogContext>} */ (createContext)();
