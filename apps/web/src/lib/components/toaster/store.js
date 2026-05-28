import { writable } from "svelte/store";

/**
 * @typedef {Object} ToastData
 * @property {string} id
 * @property {string} message
 * @property {number} duration
 *
 * @property {string} [description]
 * @property {Omit<import('./toast').ToastProps, 'toast'>} [props]
 */

/**
 * @typedef {import('svelte/store').Writable<ToastData[]>} ToasterStore
 */

/**
 * @typedef {Object} ToasterFns
 * @property {(message: string, data?: Partial<Omit<ToastData, 'message' | 'id'>>) => void} push
 * @property {(id: string) => void} remove
 */

// @ts-expect-error
export const toaster = /** @type {ToasterStore & ToasterFns} */ (writable([]));

toaster.push = enqueue;
toaster.remove = dequeue;

/** @type {ToasterFns['push']} */
function enqueue(message, data) {
    /** @type {ToastData} */
    const toast = Object.assign(
        {
            id: crypto.randomUUID(),
            message,
            duration: 3000,
            description: undefined,
            props: undefined,
        },
        data,
    );

    toaster.update((queue) => {
        queue.push(toast);
        return queue;
    });
}

/** @type {ToasterFns['remove']} */
function dequeue(id) {
    toaster.update((queue) => queue.filter((toast) => toast.id !== id));
}
