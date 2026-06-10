import { getRssFeedsQueryOptions } from "$lib/query/rss-feeds";

/** @type {import("./$types").PageLoad} */
export async function load({ parent }) {
    const { queryClient, ky } = await parent();

    await queryClient.prefetchQuery(getRssFeedsQueryOptions(ky));
}
