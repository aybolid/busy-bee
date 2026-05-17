<script>
	import { useCellContext } from '$lib/hooks/table';

	/** @type {import('@tanstack/svelte-table').Cell<import('$lib/hooks/table').AppTableFeatures, any, string>} */
	const cell = useCellContext();
</script>

<span class="font-mono text-xs text-nowrap text-base-content/60">{cell.getValue()}</span>
