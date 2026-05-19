import { paginationSchema } from "$lib/api/common";
import { getArticlesQueryOptions } from "$lib/query/articles";
import z from "zod";

const searchParamsSchema = z.object({
    ...paginationSchema.shape,
});

/** @type {import("./$types").PageLoad} */
export async function load({ parent, url }) {
    const { queryClient, ky } = await parent();

    const searchParams = searchParamsSchema.parse(Object.fromEntries(url.searchParams));

    await queryClient.ensureQueryData(
        getArticlesQueryOptions(ky, {
            searchParams: { limit: searchParams.limit, page_index: searchParams.page_index },
        }),
    );

    return { searchParams };
}
