import { getArticlesQueryOptions } from '$lib/query/articles';

/** @type {import("./$types").PageLoad} */
export async function load({ parent }) {
	const { queryClient, ky } = await parent();

	await queryClient.ensureQueryData(getArticlesQueryOptions(ky));
}
