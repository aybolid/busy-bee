import {
    createInstructionPrompt,
    createSystemPrompt,
    deleteInstructionPrompt,
    deleteSystemPrompt,
    getInstructionPrompt,
    getInstructionPrompts,
    getSystemPrompt,
    getSystemPrompts,
    updateInstructionPrompt,
    updateSystemPrompt,
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

export function createUpdateSystemPromptMutation() {
    /** @param {Parameters<typeof updateSystemPrompt>} args */
    async function mutationFn(args) {
        return updateSystemPrompt(...args);
    }

    return createMutation(() => ({
        mutationKey: ["system_prompts", "update"],
        mutationFn,
    }));
}

////////////////////////////////////

export function createCreateInstructionPromptMutation() {
    /** @param {Parameters<typeof createInstructionPrompt>} args */
    async function mutationFn(args) {
        return createInstructionPrompt(...args);
    }

    return createMutation(() => ({
        mutationKey: ["instruction_prompts", "create"],
        mutationFn,
    }));
}

/**
 * @param {Parameters<typeof getInstructionPrompts>} args `getInstructionPrompts` function arguments.
 */
export function getInstructionPromptsQueryOptions(...args) {
    return queryOptions({
        queryKey: ["instruction_prompts"],
        queryFn: () => getInstructionPrompts(...args),
    });
}

/**
 * @param {Parameters<typeof getInstructionPrompt>} args `getInstructionPrompt` function arguments.
 */
export function getInstructionPromptQueryOptions(...args) {
    return queryOptions({
        queryKey: ["instruction_prompts", args[1]],
        queryFn: () => getInstructionPrompt(...args),
    });
}

/**
 * @param {import("@tanstack/svelte-query").QueryClient} queryClient
 */
export async function invalidateInstructionPromptsQueries(queryClient) {
    await queryClient.invalidateQueries({
        queryKey: ["instruction_prompts"],
    });
}

export function createDeleteInstructionPromptMutation() {
    /** @param {Parameters<typeof deleteInstructionPrompt>} args */
    async function mutationFn(args) {
        return deleteInstructionPrompt(...args);
    }

    return createMutation(() => ({
        mutationKey: ["instruction_prompts", "delete"],
        mutationFn,
    }));
}

export function createUpdateInstructionPromptMutation() {
    /** @param {Parameters<typeof updateInstructionPrompt>} args */
    async function mutationFn(args) {
        return updateInstructionPrompt(...args);
    }

    return createMutation(() => ({
        mutationKey: ["instruction_prompts", "update"],
        mutationFn,
    }));
}
