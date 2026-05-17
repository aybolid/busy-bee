<script>
	import { createAppColumnHelper, createAppTable, getPinningStyle } from '$lib/hooks/table';
	import { getArticlesQueryOptions } from '$lib/query/articles';
	import { createQuery } from '@tanstack/svelte-query';
	import { FlexRender, renderComponent, renderSnippet } from '@tanstack/svelte-table';

	/** @type {import('./$types').PageProps} */
	const props = $props();

	const articles = createQuery(() => getArticlesQueryOptions(props.data.ky));

	const numberFormatter = new Intl.NumberFormat('en-US');
	const totalArticles = $derived(numberFormatter.format(articles.data?.length ?? 0));

	/** @type {import('$lib/hooks/table').AppTableColumnHelper<import('$lib/api/articles').Article>} */
	const helper = createAppColumnHelper();

	const columns = helper.columns([
		helper.accessor('id', {
			header: 'ID',
			cell: (ctx) => renderComponent(ctx.cell.IdCell)
		}),
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
			header: '',
			id: 'actions',
			cell: (ctx) => renderSnippet(articlePopoverActions, ctx.row.original)
		})
	]);

	const table = createAppTable({
		columns,
		initialState: { columnPinning: { right: ['actions'], left: [] } },
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

{#snippet articlePopoverActions(/** @type {import('$lib/api/articles').Article} */ article)}
	<button
		class="btn btn-ghost btn-xs"
		popovertarget="popover-actions-{article.id}"
		style:anchor-name="--anchor-actions-{article.id}"
	>
		...
	</button>
	<ul
		class="menu dropdown w-52 menu-sm rounded-box bg-base-200 p-2"
		popover
		id="popover-actions-{article.id}"
		style:position-anchor="--anchor-actions-{article.id}"
	>
		<li><a href="/articles/{article.id}">View</a></li>
		{#if article.url && article.url.startsWith('http')}
			<li><a href={article.url} target="_blank">View external</a></li>
		{/if}
	</ul>
{/snippet}

<div class="flex justify-between gap-4 pb-8">
	<h1 class="text-2xl font-bold">Artciles</h1>

	<div class="stats overflow-visible">
		<div class="stat p-0">
			<div class="stat-title">Total articles</div>
			<div class="stat-value">{totalArticles}</div>
		</div>
	</div>
</div>

{#if articles.isLoading}
	<p>Loading...</p>
{:else if articles.isError}
	<p>Error: {articles.error.message}</p>
{:else if articles.isSuccess}
	<table.AppTable>
		<div class="w-full overflow-x-auto">
			<table class="table">
				<thead>
					{#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
						<tr>
							{#each headerGroup.headers as h (h.id)}
								<table.AppHeader header={h}>
									{#snippet children(header)}
										{@const style = getPinningStyle(
											/** @type {import('$lib/hooks/table').AnyAppTableColumn} */ (header.column)
										)}
										<th colSpan={header.colSpan} {style}>
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
										{@const style = getPinningStyle(
											/** @type {import('$lib/hooks/table').AnyAppTableColumn} */ (cell.column)
										)}
										<td {style}>
											<FlexRender {cell} />
										</td>
									{/snippet}
								</table.AppCell>
							{/each}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
		<table.PaginationControls />
	</table.AppTable>
{/if}
