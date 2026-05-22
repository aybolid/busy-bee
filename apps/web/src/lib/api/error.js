import { dev } from "$app/environment";
import z from "zod";

const baseErrorSchema = { message: z.string(), timestamp: z.coerce.date() };

const errorSchema = z.discriminatedUnion("kind", [
    z.object({ ...baseErrorSchema, kind: z.literal("message") }),
    z.object({ ...baseErrorSchema, kind: z.literal("internal"), trace_id: z.uuid() }),
    z.object({ ...baseErrorSchema, kind: z.literal("validation"), source: z.string().nullable() }),
]);

/** @typedef {z.infer<typeof errorSchema>} ApiError */

/**
 * @param {import('ky').HTTPError} error
 * @returns {ApiError | null}
 */
export function getApiError(error) {
    try {
        const data = error.data;
        return errorSchema.parse(data);
    } catch (error) {
        if (dev) {
            alert(String(error));
        }
        console.error(error);
        return null;
    }
}
