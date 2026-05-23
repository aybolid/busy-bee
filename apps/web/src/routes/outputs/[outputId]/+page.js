import { getArticleProcessingOutputQueryOptions } from "$lib/query/article-processing-outputs";

/** @type {import("./$types").PageLoad} */
export async function load({ parent, params }) {
    const { queryClient, ky } = await parent();

    const outputOptions = getArticleProcessingOutputQueryOptions(ky, {
        params: {
            id: /** @type {import('$lib/api/article-processing-outputs').ArticleProcessingOutputId} */ (
                params.outputId
            ),
        },
    });

    await queryClient.prefetchQuery(outputOptions);
}
