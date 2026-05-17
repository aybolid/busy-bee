<script lang="ts">
	import { useTableContext } from '$lib/hooks/table';

	const table = useTableContext();
</script>

<div class="flex w-full items-center justify-between gap-4 pt-4">
	<div class="join">
		<button
			class="btn join-item btn-sm"
			onclick={() => table.previousPage()}
			disabled={!table.getCanPreviousPage()}
		>
			{'<'}
		</button>
		<span class="btn btn-disabled join-item text-base-content btn-ghost btn-sm">
			Page {(table.store.state.pagination.pageIndex + 1).toLocaleString()}
		</span>
		<button
			class="btn join-item btn-sm"
			onclick={() => table.nextPage()}
			disabled={!table.getCanNextPage()}
		>
			{'>'}
		</button>
	</div>
	<select
		class="select ml-auto w-fit bg-base-200 select-sm"
		value={table.store.state.pagination.pageSize}
		onchange={(e) => {
			table.setPageSize(Number(e.currentTarget.value));
		}}
	>
		{#each [10, 20, 30, 40, 50] as size}
			<option value={size}>Show {size}</option>
		{/each}
	</select>
</div>
