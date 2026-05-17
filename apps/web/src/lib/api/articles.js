import z from 'zod';
import { unwrapData } from './common';

const articleIdSchema = z.uuidv7().brand('articleId');

/** @typedef {z.infer<typeof articleIdSchema>} ArticleId */

const textDirSchema = z.enum(['ltr', 'rtl']);

/** @typedef {z.infer<typeof textDirSchema>} TextDir */

const articleSchema = z
	.object({
		id: articleIdSchema,
		created_at: z.coerce.date(),
		updated_at: z.coerce.date(),

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
		url: z.string().nullable()
	})
	.strict();

/** @typedef {z.infer<typeof articleSchema>} Article */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 *
 * @returns {Promise<Array<Article>>} Array of articles.
 */
export async function getArticles(ky) {
	const json = await ky.get('articles').json();
	return unwrapData(z.array(articleSchema)).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: ArticleId } }} payload Request payload.
 *
 * @returns {Promise<Article>} Article.
 */
export async function getArticle(ky, payload) {
	const json = await ky.get(`articles/${payload.params.id}`).json();
	return { ...unwrapData(articleSchema).parse(json) };
}
