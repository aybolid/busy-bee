import { getArticles } from '$lib/api/articles';
import { queryOptions } from '@tanstack/svelte-query';

/**
 * @param {typeof fetch} fetch
 */
export function getArticlesQueryOptions(fetch) {
	return queryOptions({
		queryKey: ['articles'],
		queryFn: () => getArticles(fetch)
	});
}
