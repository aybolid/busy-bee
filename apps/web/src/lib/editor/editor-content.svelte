<script>
    import { onMount } from "svelte";

    /** @type {Omit<import('svelte/elements').HTMLAttributes<HTMLDivElement>, 'children' | 'id'> & { editor: import('@tiptap/core').Editor }} */
    const { editor, ...props } = $props();

    /** @type {HTMLDivElement} */
    let container;

    onMount(() => editor.mount(container));
</script>

<div {...props} bind:this={container}></div>
