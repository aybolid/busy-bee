import {
    bulkDeleteArticles,
    deleteArticle,
    getArticle,
    getArticles,
    getArticleStats,
    processArticle,
} from "$lib/api/articles";
import { createMutation, keepPreviousData, queryOptions } from "@tanstack/svelte-query";

/**
 * @param {Parameters<typeof getArticles>} args `getArticles` function arguments.
 */
export function getArticlesQueryOptions(...args) {
    return queryOptions({
        queryKey: ["articles", args[1]],
        queryFn: () => getArticles(...args),
        placeholderData: keepPreviousData,
    });
}

/**
 * @param {Parameters<typeof getArticle>} args `getArticle` function arguments.
 */
export function getArticleQueryOptions(...args) {
    return queryOptions({
        queryKey: ["articles", args[1]],
        queryFn: () => getArticle(...args),
    });
}

/**
 * @param {import("@tanstack/svelte-query").QueryClient} queryClient
 */
export async function invalidateArticlesQueries(queryClient) {
    await queryClient.invalidateQueries({
        queryKey: ["articles"],
    });
}

/**
 * @param {Parameters<typeof getArticleStats>} args `getArticleStats` function arguments.
 */
export function getArticleStatsQueryOptions(...args) {
    return queryOptions({
        queryKey: ["articles/stats"],
        queryFn: () => getArticleStats(...args),
    });
}

/**
 * @param {import("@tanstack/svelte-query").QueryClient} queryClient
 */
export async function invalidateArticleStatsQueries(queryClient) {
    await queryClient.invalidateQueries({
        queryKey: ["articles/stats"],
    });
}

export function createDeleteArticleMutation() {
    /** @param {Parameters<typeof deleteArticle>} args */
    async function mutationFn(args) {
        return deleteArticle(...args);
    }

    return createMutation(() => ({
        mutationKey: ["articles", "delete"],
        mutationFn,
    }));
}

export function createProcessArticleMutation() {
    /** @param {Parameters<typeof processArticle>} args */
    async function mutationFn(args) {
        return processArticle(...args);
    }

    return createMutation(() => ({
        mutationKey: ["articles", "process"],
        mutationFn,
    }));
}

export function createBulkDeleteArticlesMutation() {
    /** @param {Parameters<typeof bulkDeleteArticles>} args */
    async function mutationFn(args) {
        return bulkDeleteArticles(...args);
    }

    return createMutation(() => ({
        mutationKey: ["articles", "bulk", "delete"],
        mutationFn,
    }));
}
