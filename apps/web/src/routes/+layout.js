import { browser } from '$app/environment';
import { QueryClient } from '@tanstack/svelte-query';

/** @type {import('./$types').LayoutLoad} */
export function load() {
	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser,
				staleTime: 60 * 1000,
				retry: false
			}
		}
	});

	return { queryClient };
}
