import { createRssFeed, getRssFeeds } from "$lib/api/rss-feeds";
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
