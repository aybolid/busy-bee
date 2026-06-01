import { createRssFeed, getRssFeeds, deleteRssFeed } from "$lib/api/rss-feeds";
import { createMutation, queryOptions } from "@tanstack/svelte-query";

/**
 * @param {Parameters<typeof getRssFeeds>} args `getRssFeeds` function arguments.
 */
export function getRssFeedsQueryOptions(...args) {
    return queryOptions({
        queryKey: ["rss_feeds"],
        queryFn: () => getRssFeeds(...args),
    });
}

/** create-create :) */
export function createCreateRssFeedMutation() {
    /** @param {Parameters<typeof createRssFeed>} args */
    async function mutationFn(args) {
        return createRssFeed(...args);
    }

    return createMutation(() => ({
        mutationKey: ["rss_feeds", "create"],
        mutationFn,
    }));
}

export function createDeleteRssFeedMutation() {
    /** @param {Parameters<typeof deleteRssFeed>} args */
    async function mutationFn(args) {
        return deleteRssFeed(...args);
    }

    return createMutation(() => ({
        mutationKey: ["rss_feeds", "delete"],
        mutationFn,
    }));
}
