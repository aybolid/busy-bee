import {
    getInstructionPromptsQueryOptions,
    getSystemPromptsQueryOptions,
} from "$lib/query/prompts";

/** @type {import("./$types").PageLoad} */
export async function load({ parent }) {
    const { queryClient, ky } = await parent();

    await Promise.all([
        queryClient.prefetchQuery(getSystemPromptsQueryOptions(ky)),
        queryClient.prefetchQuery(getInstructionPromptsQueryOptions(ky)),
    ]);
}
