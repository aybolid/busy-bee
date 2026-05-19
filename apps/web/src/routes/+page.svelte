<script>
	import { getArticlesQueryOptions } from '$lib/query/articles';
	import { createQuery } from '@tanstack/svelte-query';

	/** @type {import('./$types').PageProps} */
	const props = $props();

	/** @type {import('$lib/api/articles').GetArticlesSearchParams} */
	const getArticlesSearchParams = $derived({
		page_index: props.data.searchParams.page_index,
		limit: props.data.searchParams.limit,
	});

	const articles = createQuery(() =>
		getArticlesQueryOptions(props.data.ky, { searchParams: getArticlesSearchParams }),
	);
</script>

<div class="flex justify-between gap-4 pb-8">
	<h1 class="text-2xl font-bold">Artciles</h1>
</div>

{#if articles.isLoading}
	<p>Loading...</p>
{:else if articles.isError}
	<p>Error: {articles.error.message}</p>
{:else if articles.isSuccess}
	<section class="grid grid-cols-2 gap-4">
		{#each articles.data.data as article (article.id)}
			<div>
				{#if article.image && article.image.startsWith('http')}
					<figure>
						<img src={article.image} alt="Shoes" />
					</figure>
				{/if}
				<div>
					<h2>{article.title}</h2>
					{#if article.excerpt}
						<p class="line-clamp-3">
							{article.excerpt}
						</p>
					{/if}
					<div class="flex flex-wrap items-center justify-end">
						<a href="/articles/{article.id}">View</a>
					</div>
				</div>
			</div>
		{/each}
	</section>
{/if}
