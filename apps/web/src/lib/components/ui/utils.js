import { twMerge } from "tailwind-merge";
import { clsx } from "clsx";

/**
 * @param {import('clsx').ClassArray} values
 *
 * @returns {string}
 */
export function cn(...values) {
    return twMerge(clsx(values));
}
