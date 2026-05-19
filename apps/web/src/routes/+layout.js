import { browser } from "$app/environment";
import { QueryClient } from "@tanstack/svelte-query";
import _ky from "ky";

/** @type {import('./$types').LayoutLoad} */
export function load({ url, fetch }) {
    const queryClient = new QueryClient({
        defaultOptions: {
            queries: {
                enabled: browser,
                staleTime: 60 * 1000,
                retry: false,
            },
        },
    });

    const ky = _ky.create({ baseUrl: `${url.origin}/api/`, retry: 0, fetch });

    return { queryClient, ky };
}
