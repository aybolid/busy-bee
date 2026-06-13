import { articleStatusSchema } from "$lib/api/articles";
import { paginationSchema } from "$lib/api/common";
import { rssFeedIdSchema } from "$lib/api/rss-feeds";
import { getArticlesQueryOptions } from "$lib/query/articles";
import z from "zod";

const searchParamsSchema = z.object({
    ...paginationSchema.shape,
    query: z.string().min(2).max(255).optional(),
    rss_feed_id: rssFeedIdSchema.optional(),
    status: articleStatusSchema.optional(),
});

/** @type {import("./$types").PageLoad} */
export async function load({ parent, url }) {
    const { queryClient, ky } = await parent();

    const searchParams = searchParamsSchema.parse(Object.fromEntries(url.searchParams));

    await queryClient.prefetchQuery(
        getArticlesQueryOptions(ky, {
            searchParams: {
                limit: searchParams.limit,
                page_index: searchParams.page_index,
                query: searchParams.query,
                rss_feed_id: searchParams.rss_feed_id,
                status: searchParams.status,
            },
        }),
    );

    return { searchParams };
}
