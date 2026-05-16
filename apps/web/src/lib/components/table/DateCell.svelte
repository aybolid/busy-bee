<script>
	import { useCellContext } from '$lib/hooks/table';
	import dayjs from 'dayjs';

	/** @type {import('@tanstack/svelte-table').Cell<import('$lib/hooks/table').AppTableFeatures, any, dayjs.ConfigType>} */
	const cell = useCellContext();

	const value = cell.getValue();
</script>

{#if value}
	<span class="badge badge-ghost badge-sm text-nowrap">
		{dayjs(value).format('MMM DD, YYYY, HH:mm')}
	</span>
{/if}
