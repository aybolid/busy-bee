export const RSS_CATEGORIES = /** @type {const} */ ([
    "Android Development",
    "Android",
    "Apple",
    "Architecture",
    "Beauty",
    "Books",
    "Business & Economy",
    "Cars",
    "Cricket",
    "DIY",
    "Fashion",
    "Food",
    "Football",
    "Funny",
    "Gaming",
    "History",
    "Interior design",
    "Movies",
    "Music",
    "News",
    "Personal finance",
    "Photography",
    "Programming",
    "Science",
    "Space",
    "Sports",
    "Startups",
    "Tech",
    "Television",
    "Tennis",
    "Travel",
    "UI - UX",
    "Web Development",
    "iOS Development",
]);

/** @typedef {typeof RSS_CATEGORIES[number]} RssCategory */

/**
 * @param {import('ky').KyInstance} ky
 * @param {{ category: RssCategory }} payload
 *
 * @returns {Promise<Blob>} OPML file as a Blob
 */
export async function getRssCategoryOpmlFile(ky, payload) {
    const extended = ky.extend({
        baseUrl:
            "https://raw.githubusercontent.com/plenaryapp/awesome-rss-feeds/refs/heads/master/recommended/with_category/",
    });
    return extended.get(`${payload.category}.opml`).blob();
}
