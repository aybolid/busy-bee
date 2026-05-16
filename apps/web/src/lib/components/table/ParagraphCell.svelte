<script>
	import { useCellContext } from '$lib/hooks/table';

	/** @type {import('@tanstack/svelte-table').Cell<import('$lib/hooks/table').AppTableFeatures, any, string>} */
	const cell = useCellContext();
</script>

<p class="line-clamp-2 text-base-content/60">{cell.getValue()}</p>
