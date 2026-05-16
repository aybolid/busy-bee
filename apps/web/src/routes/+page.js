import { getArticlesQueryOptions } from '$lib/query/articles';

/** @type {import("./$types").PageLoad} */
export async function load({ parent }) {
	const { queryClient } = await parent();

	await queryClient.ensureQueryData(getArticlesQueryOptions(fetch));
}
