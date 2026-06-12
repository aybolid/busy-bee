import {
    createSystemPrompt,
    deleteSystemPrompt,
    getSystemPrompt,
    getSystemPrompts,
} from "$lib/api/prompts";
import { createMutation, queryOptions } from "@tanstack/svelte-query";

export function createCreateSystemPromptMutation() {
    /** @param {Parameters<typeof createSystemPrompt>} args */
    async function mutationFn(args) {
        return createSystemPrompt(...args);
    }

    return createMutation(() => ({
        mutationKey: ["system_prompts", "create"],
        mutationFn,
    }));
}

/**
 * @param {Parameters<typeof getSystemPrompts>} args `getSystemPrompts` function arguments.
 */
export function getSystemPromptsQueryOptions(...args) {
    return queryOptions({
        queryKey: ["system_prompts"],
        queryFn: () => getSystemPrompts(...args),
    });
}

/**
 * @param {Parameters<typeof getSystemPrompt>} args `getSystemPrompt` function arguments.
 */
export function getSystemPromptQueryOptions(...args) {
    return queryOptions({
        queryKey: ["system_prompts", args[1]],
        queryFn: () => getSystemPrompt(...args),
    });
}

/**
 * @param {import("@tanstack/svelte-query").QueryClient} queryClient
 */
export async function invalidateSystemPromptsQueries(queryClient) {
    await queryClient.invalidateQueries({
        queryKey: ["system_prompts"],
    });
}

export function createDeleteSystemPromptMutation() {
    /** @param {Parameters<typeof deleteSystemPrompt>} args */
    async function mutationFn(args) {
        return deleteSystemPrompt(...args);
    }

    return createMutation(() => ({
        mutationKey: ["system_prompts", "delete"],
        mutationFn,
    }));
}
