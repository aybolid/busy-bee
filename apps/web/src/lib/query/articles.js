import { deleteArticle, getArticle, getArticles, getArticleStats } from "$lib/api/articles";
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
 * @param {Parameters<typeof getArticleStats>} args `getArticleStats` function arguments.
 */
export function getArticleStatsQueryOptions(...args) {
    return queryOptions({
        queryKey: ["articles", "stats"],
        queryFn: () => getArticleStats(...args),
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
