import { getSystemPromptsQueryOptions } from "$lib/query/prompts";

/** @type {import("./$types").PageLoad} */
export async function load({ parent }) {
    const { queryClient, ky } = await parent();

    await queryClient.prefetchQuery(getSystemPromptsQueryOptions(ky));
}
