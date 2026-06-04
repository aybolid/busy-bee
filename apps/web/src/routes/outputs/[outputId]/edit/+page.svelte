<script>
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
    import StickyBar from "$lib/components/ui/sticky-bar.svelte";
    import { createQuery } from "@tanstack/svelte-query";
    import { getArticleProcessingOutputQueryOptions } from "$lib/query/article-processing-outputs";
    import Action from "$lib/components/ui/action.svelte";
    import Editor from "$lib/editor/editor.svelte";
    import EditorContent from "$lib/editor/editor-content.svelte";
    import EditorToolbar from "$lib/editor/editor-toolbar.svelte";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    const outputId =
        /** @type {import('$lib/api/article-processing-outputs').ArticleProcessingOutputId} */ (
            $derived(props.params.outputId)
        );

    const output = createQuery(() =>
        getArticleProcessingOutputQueryOptions(props.data.ky, { params: { id: outputId } }),
    );
</script>

{#if output.isPending}
    <Pending />
{:else if output.isError}
    <ErrorAlert error={output.error} />
{:else if output.isSuccess}
    <div class="space-y-8">
        <Editor initialContent={output.data.output_text}>
            {#snippet children({ editor })}
                <EditorToolbar {editor} position="top" class="z-20" />
                <EditorContent
                    {editor}
                    class="mx-auto prose max-w-4xl prose-neutral dark:prose-invert"
                />
            {/snippet}
        </Editor>

        <StickyBar>
            <div class="space-x-2">
                <Action button variant="outline">Cancel</Action>
                <Action button>Save</Action>
            </div>
        </StickyBar>
    </div>
{/if}
