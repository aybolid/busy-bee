import { getOutputQueryOptions } from "$lib/query/outputs";

/** @type {import("./$types").PageLoad} */
export async function load({ parent, params }) {
    const { queryClient, ky } = await parent();

    const outputOptions = getOutputQueryOptions(ky, {
        params: {
            id: /** @type {import('$lib/api/outputs').OutputId} */ (params.outputId),
        },
    });

    await queryClient.prefetchQuery(outputOptions);
}
