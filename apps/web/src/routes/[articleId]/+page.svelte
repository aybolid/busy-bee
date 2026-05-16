<script>
	import { getArticleQueryOptions } from '$lib/query/articles';
	import { createQuery } from '@tanstack/svelte-query';
	import dayjs from 'dayjs';

	/** @type {import('./$types').PageProps} */
	const props = $props();

	const articleId = /** @type {import('$lib/api/articles').ArticleId} */ (
		$derived(props.params.articleId)
	);

	const article = createQuery(() =>
		getArticleQueryOptions(props.data.api, { params: { id: articleId } })
	);
</script>

{#if article.isLoading}
	<p>Loading...</p>
{:else if article.isError}
	<p>Error: {article.error.message}</p>
{:else if article.isSuccess}
	<div class="flex flex-col gap-2 pb-8">
		<h1 class="text-2xl font-bold">{article.data.title}</h1>
		<div class="flex flex-wrap gap-2">
			{#if article.data.byline}
				<span class="badge badge-primary">{article.data.byline}</span>
			{/if}
			{#if article.data.published_time}
				<span class="badge badge-ghost">
					{dayjs(article.data.published_time).format('MMM DD, YYYY, HH:mm')}
				</span>
			{/if}
		</div>
	</div>

	<article class="prose max-w-full">{@html article.data.content}</article>
{/if}
