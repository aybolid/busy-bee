import { dev } from "$app/environment";
import { toaster } from "$lib/components/toaster/store";
import { invalidateOutputsQueries } from "$lib/query/outputs";
import { invalidateArticlesQueries, invalidateArticleStatsQueries } from "$lib/query/articles";
import { invalidateRssFeedsQueries } from "$lib/query/rss-feeds";
import z from "zod";

const notificationDataSchema = z
    .object({
        variant: z.enum(["info", "error"]),
        title: z.string(),
        description: z.string().nullable(),
    })
    .strict();

const refetchTriggerTypeSchema = z.enum(["articles", "rss_feeds", "outputs"]);

/** @typedef {z.infer<typeof refetchTriggerTypeSchema>} RefetchTriggerType */

/** @typedef {z.infer<typeof notificationDataSchema>} NotificationData */

/**
 * @param {import("@tanstack/svelte-query").QueryClient} queryClient
 *
 * @returns {() => void} Cleanup function
 */
export function sseListener(queryClient) {
    const sse = new EventSource(`${window.location.origin}/api/sse`);

    sse.addEventListener("notification", (e) => {
        const data = parseNotificationData(e.data);
        if (data) {
            toaster.push(data.title, {
                description: data.description || undefined,
                props: { variant: data.variant === "error" ? "destructive" : "default" },
            });
        }
    });

    sse.addEventListener("refetch_trigger", (e) => {
        const data = parseRefetchTriggerData(e.data);
        if (data) {
            switch (data) {
                case "articles":
                    void invalidateArticlesQueries(queryClient);
                    void invalidateArticleStatsQueries(queryClient);
                    break;
                case "rss_feeds":
                    void invalidateRssFeedsQueries(queryClient);
                    break;
                case "outputs":
                    void invalidateOutputsQueries(queryClient);
                    break;
            }
        }
    });

    return () => {
        sse.close();
    };
}

/**
 * @param {string} data
 * @returns {RefetchTriggerType | null}
 */
function parseRefetchTriggerData(data) {
    try {
        return refetchTriggerTypeSchema.parse(data);
    } catch (err) {
        if (dev) {
            alert(String(err));
        }
        return null;
    }
}

/**
 * @param {string} data
 * @returns {NotificationData | null}
 */
function parseNotificationData(data) {
    try {
        return notificationDataSchema.parse(JSON.parse(data));
    } catch (err) {
        if (dev) {
            alert(String(err));
        }
        return null;
    }
}
