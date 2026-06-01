import { getRssCategoryOpmlFile } from "$lib/api/awesome-rss-feeds";
import { queryOptions } from "@tanstack/svelte-query";
import z from "zod";

const awesomeRssFeedSchema = z.object({
    url: z.httpUrl(),
    title: z.string().min(1),
    description: z.string().optional(),
});

/** @typedef {z.infer<typeof awesomeRssFeedSchema>} AwesomeRssFeed */

/**
 * @param {Parameters<typeof getRssCategoryOpmlFile>} args `getRssCategoryOpmlFile` function arguments.
 */
export function getAwesomeRssFeedsQueryOptions(...args) {
    return queryOptions({
        queryKey: ["awesome_rss_feeds", args[1]],
        queryFn: async () => {
            const text = await getRssCategoryOpmlFile(...args).then((file) => file.text());

            const parser = new DOMParser();
            const xml = parser.parseFromString(text, "text/xml");

            const outlines = xml.querySelectorAll("outline");

            /** @type {AwesomeRssFeed[]} */
            const feeds = [];

            for (const outline of outlines) {
                const url = outline.getAttribute("xmlUrl");
                const type = outline.getAttribute("type");
                if (!url || type !== "rss") continue;

                const feed = {
                    url,
                    title: outline.getAttribute("title"),
                    description: outline.getAttribute("description") || undefined,
                };

                const result = await awesomeRssFeedSchema.safeParseAsync(feed);
                if (result.success) {
                    feeds.push(result.data);
                }
            }

            return feeds;
        },
        staleTime: Infinity,
    });
}
