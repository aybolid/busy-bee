import { deleteOutput, getOutput, getOutputs } from "$lib/api/outputs";
import { createMutation, keepPreviousData, queryOptions } from "@tanstack/svelte-query";

/**
 * @param {Parameters<typeof getOutputs>} args `getOutputs` function arguments.
 */
export function getOutputsQueryOptions(...args) {
    return queryOptions({
        queryKey: ["outputs", args[1]],
        queryFn: () => getOutputs(...args),
        placeholderData: keepPreviousData,
    });
}

/**
 * @param {Parameters<typeof getOutput>} args `getOutput` function arguments.
 */
export function getOutputQueryOptions(...args) {
    return queryOptions({
        queryKey: ["outputs", args[1]],
        queryFn: () => getOutput(...args),
    });
}

/**
 * @param {import("@tanstack/svelte-query").QueryClient} queryClient
 */
export function invalidateOutputsQueries(queryClient) {
    void queryClient.invalidateQueries({
        queryKey: ["outputs"],
    });
}

export function createDeleteOutputMutation() {
    /** @param {Parameters<typeof deleteOutput>} args */
    async function mutationFn(args) {
        return deleteOutput(...args);
    }

    return createMutation(() => ({
        mutationKey: ["outputs", "delete"],
        mutationFn,
    }));
}
