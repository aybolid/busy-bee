<script>
	import { getArticlesQueryOptions } from '$lib/query/articles';
	import { createQuery } from '@tanstack/svelte-query';

	const articles = createQuery(() => getArticlesQueryOptions(fetch));
</script>

{#if articles.isLoading}
	<p>Loading...</p>
{:else if articles.isError}
	<p>Error: {articles.error.message}</p>
{:else if articles.isSuccess}{/if}
