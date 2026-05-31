import { dev } from "$app/environment";
import { toaster } from "$lib/components/toaster/store";
import z from "zod";

const notificationDataSchema = z
    .object({
        variant: z.enum(["info", "error"]),
        title: z.string(),
        description: z.string().nullable(),
    })
    .strict();

/** @typedef {z.infer<typeof notificationDataSchema>} NotificationData */

/**
 * @returns {() => void} Cleanup function
 */
export function sseListener() {
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

    return () => {
        sse.close();
    };
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
