import z from "zod";

/**
 * @template {z.ZodType} T
 * @param {T} type `zod` schema of the value to unwrap.
 */
export function unwrapData(type) {
    return (
        z
            .object({ data: type })
            .strict()
            // @ts-expect-error
            .transform((v) => v.data)
    );
}

export const paginationMetaSchema = z.object({});

/** @typedef {z.infer<typeof paginationMetaSchema>} PaginationMeta */

/**
 * @template {z.ZodType} T
 * @param {T} type `zod` schema of the data.
 */
export function dataWithPaginationMeta(type) {
    return z.object({ data: type, meta: paginationMetaSchema }).strict();
}

export const paginationSchema = z.object({
    page_index: z.coerce.number().nonnegative().default(0),
    limit: z.coerce.number().int().max(255).positive().default(20),
});

/** @typedef {z.infer<typeof paginationSchema>} Pagination */
