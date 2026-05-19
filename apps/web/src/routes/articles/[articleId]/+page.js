import { getArticleQueryOptions } from '$lib/query/articles';

/** @type {import("./$types").PageLoad} */
export async function load({ parent, params }) {
	const { queryClient, ky } = await parent();

	const articleOptions = getArticleQueryOptions(ky, {
		params: { id: /** @type {import('$lib/api/articles').ArticleId} */ (params.articleId) },
	});

	await queryClient.ensureQueryData(articleOptions);
}
