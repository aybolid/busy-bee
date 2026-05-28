import z from "zod";
import { unwrapData } from "./common";

const rssFeedIdSchema = z.uuidv7().brand("rssFeedId");

/** @typedef {z.infer<typeof rssFeedIdSchema>} RssFeedId */

const baseRssFeedSchema = {
    id: rssFeedIdSchema,
    created_at: z.coerce.date(),
    updated_at: z.coerce.date(),

    url: z.url(),
    max_concurrent_requests: z.int().positive(),
    fetch_interval_seconds: z.int().positive(),
};

const rssFeedSchema = z.discriminatedUnion("status", [
    z.object({ ...baseRssFeedSchema, status: z.literal("healthy") }).strict(),
    z
        .object({ ...baseRssFeedSchema, status: z.literal("error"), error_reason: z.string() })
        .strict(),
]);

/** @typedef {z.infer<typeof rssFeedSchema>} RssFeed */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 *
 * @returns {Promise<Array<RssFeed>>} Array of RSS feeds.
 */
export async function getRssFeeds(ky) {
    const json = await ky.get("rss_feeds").json();
    return unwrapData(z.array(rssFeedSchema)).parse(json);
}
