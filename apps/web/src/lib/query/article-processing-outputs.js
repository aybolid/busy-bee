import {
    getArticleProcessingOutput,
    getArticleProcessingOutputs,
} from "$lib/api/article-processing-outputs";
import { keepPreviousData, queryOptions } from "@tanstack/svelte-query";

/**
 * @param {Parameters<typeof getArticleProcessingOutputs>} args `getArticleProcessingOutputs` function arguments.
 */
export function getArticleProcessingOutputsQueryOptions(...args) {
    return queryOptions({
        queryKey: ["article_processing_outputs", args[1]],
        queryFn: () => getArticleProcessingOutputs(...args),
        placeholderData: keepPreviousData,
    });
}

/**
 * @param {Parameters<typeof getArticleProcessingOutput>} args `getArticleProcessingOutput` function arguments.
 */
export function getArticleProcessingOutputQueryOptions(...args) {
    return queryOptions({
        queryKey: ["article_processing_output", args[1]],
        queryFn: () => getArticleProcessingOutput(...args),
    });
}
