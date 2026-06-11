<script>
    import { cn } from "$lib/components/ui/utils";
    import { onMount } from "svelte";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & { editor: import('@tiptap/core').Editor }} */
    const { children, editor, ...props } = $props();

    /** @type {HTMLDivElement} */
    let container;

    onMount(() => editor.mount(container));
</script>

<div
    onclick={() => editor.chain().focus().run()}
    {...props}
    class={cn(
        "prose max-w-full prose-app w-full space-y-6 min-w-0 rounded-lg border border-input bg-transparent dark:bg-input/30 p-4 text-base transition-colors outline-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40",
        props.class,
    )}
    bind:this={container}
>
    {@render children?.()}
</div>

<style>
    :global {
        .tiptap:focus {
            outline: none;
        }

        .tiptap p.is-editor-empty:first-child::before {
            color: var(--color-muted-foreground);
            content: attr(data-placeholder);
            float: left;
            height: 0;
            pointer-events: none;
        }
    }
</style>
