<script>
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
    import StickyBar from "$lib/components/ui/sticky-bar.svelte";
    import { createQuery } from "@tanstack/svelte-query";
    import { getOutputQueryOptions } from "$lib/query/outputs";
    import Action from "$lib/components/ui/action.svelte";
    import Editor from "$lib/editor/editor.svelte";
    import EditorContent from "$lib/editor/editor-content.svelte";
    import EditorToolbar from "$lib/editor/editor-toolbar.svelte";
    import { beforeNavigate } from "$app/navigation";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    const outputId = /** @type {import('$lib/api/outputs').OutputId} */ (
        $derived(props.params.outputId)
    );

    const output = createQuery(() =>
        getOutputQueryOptions(props.data.ky, { params: { id: outputId } }),
    );

    /** @type {import('@tiptap/core').Editor} */
    let editorInstance;

    /** @type {import('@tiptap/core').Editor['state']['doc']} */
    let initalDoc;
    let isDirty = $state(false);

    /** @type {import('svelte/elements').EventHandler<BeforeUnloadEvent, Window>} */
    function handleBeforeUnload(e) {
        if (isDirty) {
            e.preventDefault();
        }
    }

    beforeNavigate(({ cancel }) => {
        if (isDirty) {
            const shouldLeave = confirm(
                "You have unsaved changes. Are you sure you want to leave?",
            );
            if (!shouldLeave) {
                cancel();
            }
        }
    });
</script>

<svelte:window onbeforeunload={handleBeforeUnload} />

{#if output.isPending}
    <Pending />
{:else if output.isError}
    <ErrorAlert error={output.error} />
{:else if output.isSuccess}
    <div class="space-y-8">
        <Editor
            options={{
                content: output.data.text,
                editorProps: { attributes: { class: "focus:outline-none" } },
                onCreate: ({ editor }) => {
                    initalDoc = editor.state.doc;
                    editorInstance = editor;
                },
                onUpdate: ({ editor }) => (isDirty = !editor.state.doc.eq(initalDoc)),
            }}
        >
            {#snippet children({ editor })}
                <EditorToolbar {editor} position="top" class="z-20" />
                <EditorContent {editor} class="mx-auto prose max-w-4xl prose-app" />
            {/snippet}
        </Editor>

        <StickyBar class="gap-2">
            <Action anchor variant="outline" href="/outputs/{outputId}">Cancel</Action>
            <Action
                button
                disabled={!isDirty}
                onclick={() => {
                    console.log(editorInstance.getMarkdown());
                }}
            >
                Save
            </Action>
        </StickyBar>
    </div>
{/if}
