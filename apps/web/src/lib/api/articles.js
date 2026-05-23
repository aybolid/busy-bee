import z from "zod";
import { dataWithPaginationMeta, paginationSchema, unwrapData } from "./common";

export const articleIdSchema = z.uuidv7().brand("articleId");

/** @typedef {z.infer<typeof articleIdSchema>} ArticleId */

const textDirSchema = z.enum(["ltr", "rtl"]);

/** @typedef {z.infer<typeof textDirSchema>} TextDir */

const articleStatusSchema = z.enum(["new", "pending", "processed", "error"]);

/** @typedef {z.infer<typeof articleStatusSchema>} ArticleStatus */

const articleSchema = z
    .object({
        id: articleIdSchema,
        created_at: z.coerce.date(),
        updated_at: z.coerce.date(),

        status: articleStatusSchema,

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
        image: z.string().nullable(),
        favicon: z.string().nullable(),
        url: z.string().nullable(),
    })
    .strict();

/** @typedef {z.infer<typeof articleSchema>} Article */

const articleStatsSchema = z.object({
    total: z.int().nonnegative(),
    new: z.int().nonnegative(),
    pending: z.int().nonnegative(),
    processed: z.int().nonnegative(),
    error: z.int().nonnegative(),
});

/** @typedef {z.infer<typeof articleStatsSchema>} ArticleStats */

const getArticlesSearchParamsSchema = z
    .object({
        ...paginationSchema.shape,
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
