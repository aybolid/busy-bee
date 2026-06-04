<script>
    import { Editor } from "@tiptap/core";
    import { Markdown } from "@tiptap/markdown";
    import StarterKit from "@tiptap/starter-kit";
    import { onDestroy, onMount } from "svelte";

    /**
     * @typedef {Object} EditorProps
     *
     * @property {import('svelte').Snippet<[{ editor: Editor }]>} [children] The editor's children.
     * @property {Partial<Omit<import('@tiptap/core').EditorOptions, 'extensions' | 'contentType'>>} [options]
     */

    /** @type {EditorProps} */
    let { children, options } = $props();

    /** @type {{ editor: Editor | null }} */
    let state = $state({
        editor: null,
    });

    onMount(() => {
        state.editor = new Editor({
            ...options,
            extensions: [StarterKit, Markdown],
            contentType: "markdown",
            onTransaction: (event) => {
                state = { editor: event.editor };
                options?.onTransaction?.(event);
            },
        });
    });

    onDestroy(() => {
        state.editor?.destroy();
    });
</script>

{#if state.editor}
    {@render children?.({ editor: state.editor })}
{/if}
