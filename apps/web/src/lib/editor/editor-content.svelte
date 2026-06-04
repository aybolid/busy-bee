<script>
    import { onMount } from "svelte";

    const id = $props.id();

    /** @type {Omit<import('svelte/elements').HTMLAttributes<HTMLDivElement>, 'children' | 'id'> & { editor: import('@tiptap/core').Editor }} */
    const { editor, ...props } = $props();

    onMount(() => {
        const element = document.querySelector(`#${id}`);
        if (element) {
            editor.mount(element);
        }
    });
</script>

<div {...props} {id}></div>
