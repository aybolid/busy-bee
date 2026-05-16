import { getArticleQueryOptions, getArticlesQueryOptions } from '$lib/query/articles';

/** @type {import("./$types").PageLoad} */
export async function load({ parent, params }) {
	const { queryClient, api } = await parent();

	const articlesOptions = getArticlesQueryOptions(api);
	const cachedArticles = queryClient.getQueryData(articlesOptions.queryKey) ?? [];
	const matchingArticle = cachedArticles.find((a) => a.id === params.articleId);

	const articleOptions = getArticleQueryOptions(api, {
		params: { id: /** @type {import('$lib/api/articles').ArticleId} */ (params.articleId) }
	});

	if (matchingArticle !== undefined) {
		queryClient.setQueryData(articleOptions.queryKey, matchingArticle);
	} else {
		await queryClient.ensureQueryData(articleOptions);
	}
}
