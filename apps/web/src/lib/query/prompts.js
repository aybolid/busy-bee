import { createSystemPrompt } from "$lib/api/prompts";
import { createMutation } from "@tanstack/svelte-query";

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
 * @param {import("@tanstack/svelte-query").QueryClient} queryClient
 */
export async function invalidateSystemPromptsQueries(queryClient) {
    await queryClient.invalidateQueries({
        queryKey: ["system_prompts"],
    });
}
