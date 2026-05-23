import { paginationSchema } from "$lib/api/common";
import { getArticleProcessingOutputsQueryOptions } from "$lib/query/article-processing-outputs";
import z from "zod";

const searchParamsSchema = z.object({
    ...paginationSchema.shape,
});

/** @type {import("./$types").PageLoad} */
export async function load({ parent, url }) {
    const { queryClient, ky } = await parent();

    const searchParams = searchParamsSchema.parse(Object.fromEntries(url.searchParams));

    await queryClient.prefetchQuery(
        getArticleProcessingOutputsQueryOptions(ky, {
            searchParams: { limit: searchParams.limit, page_index: searchParams.page_index },
        }),
    );

    return { searchParams };
}
