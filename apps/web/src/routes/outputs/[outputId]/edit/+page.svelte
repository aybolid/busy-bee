<script>
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
    import StickyBar from "$lib/components/ui/sticky-bar.svelte";
    import { createQuery } from "@tanstack/svelte-query";
    import {
        createUpdateOutputMutation,
        getOutputQueryOptions,
        invalidateOutputsQueries,
    } from "$lib/query/outputs";
    import Action from "$lib/components/ui/action.svelte";
    import Editor from "$lib/editor/editor.svelte";
    import EditorContent from "$lib/editor/editor-content.svelte";
    import EditorToolbar from "$lib/editor/editor-toolbar.svelte";
    import { beforeNavigate } from "$app/navigation";
    import { getGlobalContext } from "$lib/global-context";
    import Spinner from "$lib/components/ui/spinner.svelte";
    import { toaster } from "$lib/components/toaster/store";
    import { getApiError } from "$lib/api/error";
    import { isHTTPError } from "ky";
    import { createForm } from "@tanstack/svelte-form";
    import z from "zod";
    import { updateOutputJsonSchema } from "$lib/api/outputs";
    import FieldGroup from "$lib/components/ui/field/field-group.svelte";
    import Field from "$lib/components/ui/field/field.svelte";
    import FieldError from "$lib/components/ui/field/field-error.svelte";

    /** @type {import('./$types').PageProps} */
    const props = $props();
    const { queryClient } = getGlobalContext();

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

    const updateMutation = createUpdateOutputMutation();

    const form = createForm(() => ({
        defaultValues: {
            text: "",
        },
        validators: {
            onSubmit: z.object({
                text: updateOutputJsonSchema.shape.text.nonoptional(),
            }),
        },
        onSubmit: async ({ value, formApi }) => {
            await updateMutation.mutateAsync(
                [props.data.ky, { params: { id: outputId }, json: { text: value.text } }],
                {
                    onError: (err) => {
                        let description = err.message;

                        if (isHTTPError(err)) {
                            const apiError = getApiError(err);

                            if (apiError) {
                                if (apiError.kind === "validation") {
                                    formApi.setErrorMap({
                                        onSubmit: { fields: { text: apiError } },
                                    });
                                    return;
                                }

                                description = apiError.message;
                            }
                        }

                        toaster.push("Failed to save changes", {
                            description,
                            props: { variant: "destructive" },
                        });
                    },
                    onSuccess: () => {
                        toaster.push("Changes saved");
                        isDirty = false;
                        void invalidateOutputsQueries(queryClient);
                    },
                },
            );
        },
    }));
</script>

<svelte:window onbeforeunload={handleBeforeUnload} />

{#if output.isPending}
    <Pending />
{:else if output.isError}
    <ErrorAlert error={output.error} />
{:else if output.isSuccess}
    <form
        class="space-y-8"
        onsubmit={(e) => {
            e.preventDefault();
            form.setFieldValue("text", editorInstance.getMarkdown());
            form.handleSubmit();
        }}
    >
        <FieldGroup>
            <form.Field name="text">
                {#snippet children(field)}
                    {@const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid}
                    <Field data-invalid={isInvalid}>
                        <Editor
                            options={{
                                content: output.data.text,
                                onBlur: field.handleBlur,
                                editorProps: {
                                    attributes: {
                                        id: field.name,
                                        name: field.name,
                                    },
                                },
                                onCreate: ({ editor }) => {
                                    initalDoc = editor.state.doc;
                                    editorInstance = editor;
                                },
                                onUpdate: ({ editor }) =>
                                    (isDirty = !editor.state.doc.eq(initalDoc)),
                            }}
                        >
                            {#snippet children({ editor })}
                                <EditorContent {editor} aria-invalid={isInvalid}>
                                    <EditorToolbar {editor} position="top" class="z-20 w-full" />
                                </EditorContent>
                            {/snippet}
                        </Editor>

                        {#if isInvalid}
                            <FieldError errors={field.state.meta.errors} />
                        {/if}
                    </Field>
                {/snippet}
            </form.Field>
        </FieldGroup>

        <StickyBar class="gap-2">
            <Action anchor variant="outline" href="/outputs/{outputId}">Cancel</Action>
            <Action button type="submit" disabled={!isDirty || form.state.isSubmitting}>
                {#if form.state.isSubmitting}
                    <Spinner />
                {/if}
                <span>Save</span>
            </Action>
        </StickyBar>
    </form>
{/if}
