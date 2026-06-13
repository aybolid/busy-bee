import z from "zod";
import { articleIdSchema } from "./articles";
import { dataWithPaginationMeta, paginationSchema, unwrapData } from "./common";
import { formatDate, formatNumber } from "$lib/formats";

const outputIdSchema = z.uuidv7().brand("outputId");

/** @typedef {z.infer<typeof outputIdSchema>} OutputId */

const usageSchema = z
    .object({
        prompt_tokens: z.int(),
        completion_tokens: z.int(),
        total_tokens: z.int(),
    })

    .strict()
    .transform((data) => ({
        ...data,
        formattedPromptTokens: () => formatNumber(data.prompt_tokens),
        formattedCompletionTokens: () => formatNumber(data.completion_tokens),
        formattedTotalTokens: () => formatNumber(data.total_tokens),
    }))
    .readonly();

/** @typedef {z.infer<typeof usageSchema>} Usage */

const outputSchema = z
    .object({
        id: outputIdSchema,
        created_at: z.coerce.date(),
        updated_at: z.coerce.date(),
        article_id: articleIdSchema.nullable(),
        user_context: z.string().nullable(),
        text: z.string(),
        model: z.string(),
        usage: usageSchema,
    })
    .strict()
    .transform((data) => ({
        ...data,
        formattedCreatedAt: () => formatDate(data.created_at),
        formattedUpdatedAt: () => formatDate(data.updated_at),
    }))
    .readonly();

/** @typedef {z.infer<typeof outputSchema>} Output */

const getOutputsSearchParamsSchema = z
    .object({
        ...paginationSchema.shape,
    })
    .strict();

/**
 * @typedef {z.infer<typeof getOutputsSearchParamsSchema>} GetOutputsSearchParams
 */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ searchParams: GetOutputsSearchParams }} payload Request payload.
 *
 * @returns {Promise<{ data: Array<Output>, meta: import('./common').PaginationMeta }>} Array of outputs and a pagination meta.
 */
export async function getOutputs(ky, payload) {
    const json = await ky
        .get("outputs", {
            searchParams: getOutputsSearchParamsSchema.parse(payload.searchParams),
        })
        .json();
    return dataWithPaginationMeta(z.array(outputSchema)).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: OutputId } }} payload Request payload.
 *
 * @returns {Promise<Output>} Article processing output.
 */
export async function getOutput(ky, payload) {
    const json = await ky.get(`outputs/${payload.params.id}`).json();
    return unwrapData(outputSchema).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: OutputId } }} payload Request payload.
 *
 * @returns {Promise<void>}
 */
export async function deleteOutput(ky, payload) {
    await ky.delete(`outputs/${payload.params.id}`);
}

export const updateOutputJsonSchema = z
    .object({
        text: z.string().min(1, "Text should not be empty").trim().optional(),
    })
    .strict();

/** @typedef {z.infer<typeof updateOutputJsonSchema>} UpdateOutputJson */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: OutputId }, json: UpdateOutputJson }} payload Request payload.
 *
 * @returns {Promise<Output>}
 */
export async function updateOutput(ky, payload) {
    const json = await ky
        .patch(`outputs/${payload.params.id}`, {
            json: updateOutputJsonSchema.parse(payload.json),
        })
        .json();

    return unwrapData(outputSchema).parse(json);
}

export const bulkDeleteOutputsJsonSchema = z
    .object({
        ids: z.array(outputIdSchema).min(1).max(255),
    })
    .strict();

/** @typedef {z.infer<typeof bulkDeleteOutputsJsonSchema>} BulkDeleteOutputsJson */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ json: BulkDeleteOutputsJson }} payload Request payload.
 *
 * @returns {Promise<void>}
 */
export async function bulkDeleteOutputs(ky, payload) {
    await ky.post(`outputs/bulk/delete`, {
        json: bulkDeleteOutputsJsonSchema.parse(payload.json),
    });
}
