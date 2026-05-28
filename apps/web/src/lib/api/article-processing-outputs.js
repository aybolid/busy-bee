import z from "zod";
import { articleIdSchema } from "./articles";
import { dataWithPaginationMeta, paginationSchema, unwrapData } from "./common";

const articleProcessingOutputIdSchema = z.uuidv7().brand("articleProcessingOutputId");

/** @typedef {z.infer<typeof articleProcessingOutputIdSchema>} ArticleProcessingOutputId */

const articleProcessingOutputSchema = z
    .object({
        id: articleProcessingOutputIdSchema,
        created_at: z.coerce.date(),
        updated_at: z.coerce.date(),

        article_id: articleIdSchema.nullable(),

        user_context: z.string().nullable(),
        output_text: z.string(),

        model: z.string(),
        prompt_tokens: z.int().nullable(),
        prompt_cache_creation_tokens: z.int().nullable(),
        prompt_cached_tokens: z.int().nullable(),
        prompt_audio_tokens: z.int().nullable(),
        completion_tokens: z.int().nullable(),
        completion_accepted_prediction_tokens: z.int().nullable(),
        completion_rejected_prediction_tokens: z.int().nullable(),
        completion_reasoning_tokens: z.int().nullable(),
        completion_audio_tokens: z.int().nullable(),
        total_tokens: z.int().nullable(),
    })
    .strict();

/** @typedef {z.infer<typeof articleProcessingOutputSchema>} ArticleProcessingOutput */

const getArticleProcessingOutputsSearchParamsSchema = z
    .object({
        ...paginationSchema.shape,
    })
    .strict();

/**
 * @typedef {z.infer<typeof getArticleProcessingOutputsSearchParamsSchema>} GetArticleProcessingOutputsSearchParams
 */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ searchParams: GetArticleProcessingOutputsSearchParams }} payload Request payload.
 *
 * @returns {Promise<{ data: Array<ArticleProcessingOutput>, meta: import('./common').PaginationMeta }>} Array of outputs and a pagination meta.
 */
export async function getArticleProcessingOutputs(ky, payload) {
    const json = await ky
        .get("article_processing_outputs", {
            searchParams: getArticleProcessingOutputsSearchParamsSchema.parse(payload.searchParams),
        })
        .json();
    return dataWithPaginationMeta(z.array(articleProcessingOutputSchema)).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: ArticleProcessingOutputId } }} payload Request payload.
 *
 * @returns {Promise<ArticleProcessingOutput>} Article processing output.
 */
export async function getArticleProcessingOutput(ky, payload) {
    const json = await ky.get(`article_processing_outputs/${payload.params.id}`).json();
    return unwrapData(articleProcessingOutputSchema).parse(json);
}
