<script>
    import { Editor } from "@tiptap/core";
    import { Markdown } from "@tiptap/markdown";
    import StarterKit from "@tiptap/starter-kit";
    import { onDestroy, onMount } from "svelte";

    /** @type {{ children?: import('svelte').Snippet<[{ editor: Editor }]>, initialContent?: string }} */
    const { children, initialContent } = $props();

    /** @type {{ editor: Editor | null }} */
    let state = $state({
        editor: null,
    });

    onMount(() => {
        state.editor = new Editor({
            extensions: [StarterKit, Markdown],
            content: initialContent,
            contentType: "markdown",
            editorProps: {
                attributes: { class: "focus:outline-none" },
            },
            onTransaction: ({ editor }) => {
                state = { editor };
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
