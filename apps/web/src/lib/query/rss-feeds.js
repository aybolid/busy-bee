import { getRssFeeds } from "$lib/api/rss-feeds";
import { queryOptions } from "@tanstack/svelte-query";

/**
 * @param {Parameters<typeof getRssFeeds>} args `getRssFeeds` function arguments.
 */
export function getRssFeedsQueryOptions(...args) {
    return queryOptions({
        queryKey: ["rss_feeds"],
        queryFn: () => getRssFeeds(...args),
    });
}
