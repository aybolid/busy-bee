<script>
	import { createAppColumnHelper, createAppTable } from '$lib/hooks/table';
	import { getArticlesQueryOptions } from '$lib/query/articles';
	import { createQuery } from '@tanstack/svelte-query';
	import { FlexRender, renderComponent, renderSnippet } from '@tanstack/svelte-table';

	/** @type {import('./$types').PageProps} */
	const props = $props();

	const articles = createQuery(() => getArticlesQueryOptions(props.data.ky));

	/** @type {import('$lib/hooks/table').AppTableColumnHelper<import('$lib/api/articles').Article>} */
	const helper = createAppColumnHelper();

	const columns = helper.columns([
		helper.accessor('title', {
			header: 'Title',
			cell: (ctx) => renderComponent(ctx.cell.TitleCell)
		}),
		helper.accessor('byline', {
			header: 'Author',
			cell: (ctx) => renderComponent(ctx.cell.TextCell)
		}),
		// helper.accessor('excerpt', {
		// 	header: 'Description',
		// 	cell: (ctx) => renderComponent(ctx.cell.ParagraphCell)
		// }),
		helper.accessor('published_time', {
			header: 'Published',
			cell: (ctx) => renderComponent(ctx.cell.DateCell)
		}),
		helper.accessor('created_at', {
			header: 'Created',
			cell: (ctx) => renderComponent(ctx.cell.DateCell)
		}),
		helper.display({
			header: 'Actions',
			cell: (ctx) => renderSnippet(articleActions, ctx.row.original)
		})
	]);

	const table = createAppTable({
		columns,
		get data() {
			return articles.data ?? [];
		}
	});

	// IMPORTANT: Derive rows from table state so Svelte tracks the dependency.
	// We must read a $state value that changes on every table update.
	// JSON.stringify forces a deep read, ensuring Svelte sees the dependency.
	const rows = $derived.by(() => {
		JSON.stringify(table.store.state);
		return table.getRowModel().rows;
	});
</script>

{#snippet articleActions(/** @type {import('$lib/api/articles').Article} */ article)}
	<div class="dropdown">
		<div tabindex="0" role="button" class="btn btn-ghost btn-xs">...</div>
		<ul tabindex="-1" class="dropdown-content menu z-1 w-52 menu-sm rounded-box bg-base-200 p-2">
			<li><a href={`/${article.id}`}>View</a></li>
			{#if article.url}
				<li><a href={article.url} target="_blank">View external</a></li>
			{/if}
		</ul>
	</div>
{/snippet}

<div class="pb-8">
	<h1 class="text-2xl font-bold">Artciles</h1>
</div>

{#if articles.isLoading}
	<p>Loading...</p>
{:else if articles.isError}
	<p>Error: {articles.error.message}</p>
{:else if articles.isSuccess}
	<table.AppTable>
		<table class="table">
			<thead>
				{#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
					<tr>
						{#each headerGroup.headers as h (h.id)}
							<table.AppHeader header={h}>
								{#snippet children(header)}
									<th colSpan={header.colSpan}>
										{#if !header.isPlaceholder}
											<header.FlexRender {header} />
										{/if}
									</th>
								{/snippet}
							</table.AppHeader>
						{/each}
					</tr>
				{/each}
			</thead>
			<tbody>
				{#each rows as row (row.id)}
					<tr>
						{#each row.getAllCells() as c (c.id)}
							<table.AppCell cell={c}>
								{#snippet children(cell)}
									<td>
										<FlexRender {cell} />
									</td>
								{/snippet}
							</table.AppCell>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
		<table.PaginationControls />
	</table.AppTable>
{/if}
