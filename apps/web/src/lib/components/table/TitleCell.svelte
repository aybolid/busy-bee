<script>
	import { useCellContext } from '$lib/hooks/table';

	/** @type {import('@tanstack/svelte-table').Cell<import('$lib/hooks/table').AppTableFeatures, any, string>} */
	const cell = useCellContext();
</script>

<span class="line-clamp-1 font-medium">{cell.getValue()}</span>
