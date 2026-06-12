import { getSystemPromptQueryOptions } from "$lib/query/prompts";

/** @type {import("./$types").PageLoad} */
export async function load({ parent, params }) {
    const { queryClient, ky } = await parent();

    const promptOptions = getSystemPromptQueryOptions(ky, {
        params: {
            id: /** @type {import('$lib/api/prompts').SystemPromptId} */ (params.systemPromptId),
        },
    });

    await queryClient.prefetchQuery(promptOptions);
}
