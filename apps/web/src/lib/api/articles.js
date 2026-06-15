import z from "zod";
import { dataWithPaginationMeta, paginationSchema, unwrapData } from "./common";
import { rssFeedIdSchema } from "./rss-feeds";
import { formatDate, formatNumber } from "$lib/formats";
import { instructionPromptIdSchema, systemPromptIdSchema } from "./prompts";

export const articleIdSchema = z.uuidv7().brand("articleId");

/** @typedef {z.infer<typeof articleIdSchema>} ArticleId */

const textDirSchema = z.enum(["ltr", "rtl"]);

/** @typedef {z.infer<typeof textDirSchema>} TextDir */

const baseArticleSchema = {
    id: articleIdSchema,
    created_at: z.coerce.date(),
    updated_at: z.coerce.date(),
    rss_feed_id: rssFeedIdSchema,
    title: z.string(),
    byline: z.string().nullable(),
    content: z.string(),
    text_content: z.string(),
    length: z.number(),
    excerpt: z.string().nullable(),
    site_name: z.string().nullable(),
    dir: textDirSchema.nullable(),
    lang: z.string().nullable(),
    published_time: z.coerce.date().nullable(),
    modified_time: z.coerce.date().nullable(),
    image: z.url().nullable(),
    favicon: z.url().nullable(),
    url: z.url(),
};

const articleSchema = z
    .discriminatedUnion("status", [
        z
            .object({ ...baseArticleSchema, status: z.literal("error"), error_reason: z.string() })
            .strict(),
        z
            .object({ ...baseArticleSchema, status: z.enum(["new", "pending", "processed"]) })
            .strict(),
    ])
    .transform((data) => ({
        ...data,
        formattedCreatedAt: () => formatDate(data.created_at),
        formattedUpdatedAt: () => formatDate(data.updated_at),
        formattedPublishedTime: () =>
            data.published_time ? formatDate(data.published_time) : null,
        formattedModifiedTime: () => (data.modified_time ? formatDate(data.modified_time) : null),
    }))
    .readonly();

/** @typedef {z.infer<typeof articleSchema>} Article */

export const ARTICLE_STATUSES = /** @type {const} */ (["error", "new", "pending", "processed"]);

export const articleStatusSchema = z.enum(ARTICLE_STATUSES);

/** @typedef {z.infer<typeof articleStatusSchema>} ArticleStatus */

const articleStatsSchema = z
    .object({
        total: z.int().nonnegative(),
        new: z.int().nonnegative(),
        pending: z.int().nonnegative(),
        processed: z.int().nonnegative(),
        error: z.int().nonnegative(),
    })
    .strict()
    .transform((data) => ({
        ...data,
        formattedTotal: () => formatNumber(data.total),
        formattedNew: () => formatNumber(data.new),
        formattedPending: () => formatNumber(data.pending),
        formattedProcessed: () => formatNumber(data.processed),
        formattedError: () => formatNumber(data.error),
    }))
    .readonly();

/** @typedef {z.infer<typeof articleStatsSchema>} ArticleStats */

const getArticlesSearchParamsSchema = z
    .object({
        ...paginationSchema.shape,
        query: z.string().min(2).max(255).optional(),
        rss_feed_id: rssFeedIdSchema.optional(),
        status: articleStatusSchema.optional(),
    })
    .strict();

/**
 * @typedef {z.infer<typeof getArticlesSearchParamsSchema>} GetArticlesSearchParams
 */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ searchParams: GetArticlesSearchParams }} payload Request payload.
 *
 * @returns {Promise<{ data: Array<Article>, meta: import('./common').PaginationMeta }>} Array of articles and a pagination meta.
 */
export async function getArticles(ky, payload) {
    const json = await ky
        .get("articles", {
            searchParams: getArticlesSearchParamsSchema.parse(payload.searchParams),
        })
        .json();
    return dataWithPaginationMeta(z.array(articleSchema)).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 *
 * @returns {Promise<ArticleStats>} Article statistics.
 */
export async function getArticleStats(ky) {
    const json = await ky.get("articles/stats").json();
    return unwrapData(articleStatsSchema).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: ArticleId } }} payload Request payload.
 *
 * @returns {Promise<Article>} Article.
 */
export async function getArticle(ky, payload) {
    const json = await ky.get(`articles/${payload.params.id}`).json();
    return unwrapData(articleSchema).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: ArticleId } }} payload Request payload.
 *
 * @returns {Promise<void>}
 */
export async function deleteArticle(ky, payload) {
    await ky.delete(`articles/${payload.params.id}`);
}

export const processArticleJsonSchema = z
    .object({
        system_prompt_id: systemPromptIdSchema,
        instruction_ids: z.array(instructionPromptIdSchema).min(1).max(255).optional(),
        context: z.string().trim().max(500).optional(),
    })
    .strict();

/** @typedef {z.infer<typeof processArticleJsonSchema>} ProcessArticleJson */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: ArticleId }, json: ProcessArticleJson }} payload Request payload.
 *
 * @returns {Promise<void>}
 */
export async function processArticle(ky, payload) {
    await ky.post(`articles/${payload.params.id}/process`, {
        json: processArticleJsonSchema.parse(payload.json),
    });
}

export const bulkDeleteArticlesJsonSchema = z
    .object({
        ids: z.array(articleIdSchema).min(1).max(255),
    })
    .strict();

/** @typedef {z.infer<typeof bulkDeleteArticlesJsonSchema>} BulkDeleteArticlesJson */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ json: BulkDeleteArticlesJson }} payload Request payload.
 *
 * @returns {Promise<void>}
 */
export async function bulkDeleteArticles(ky, payload) {
    await ky.post(`articles/bulk/delete`, {
        json: bulkDeleteArticlesJsonSchema.parse(payload.json),
    });
}
