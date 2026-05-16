import { getArticle, getArticles } from '$lib/api/articles';
import { queryOptions } from '@tanstack/svelte-query';

/**
 * @param {Parameters<typeof getArticles>} args `getArticles` function arguments.
 */
export function getArticlesQueryOptions(...args) {
	return queryOptions({
		queryKey: ['articles'],
		queryFn: () => getArticles(...args)
	});
}

/**
 * @param {Parameters<typeof getArticle>} args `getArticle` function arguments.
 */
export function getArticleQueryOptions(...args) {
	return queryOptions({
		queryKey: ['articles', args[1]],
		queryFn: () => getArticle(...args)
	});
}
