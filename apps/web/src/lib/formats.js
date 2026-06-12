import dayjs from "dayjs";
import { NUMBER_FORMAT } from "./constants";

/**
 * @param {Exclude<dayjs.ConfigType, null | undefined>} value
 *
 * @returns {string}
 */
export function formatDate(value) {
    return dayjs(value).format("MMM DD, YYYY, HH:mm");
}

/**
 * @param {number} value
 *
 * @returns {string}
 */
export function formatNumber(value) {
    return NUMBER_FORMAT.format(value);
}
