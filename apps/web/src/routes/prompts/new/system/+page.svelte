<script>
    import StickyBar from "$lib/components/ui/sticky-bar.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import Editor from "$lib/editor/editor.svelte";
    import EditorContent from "$lib/editor/editor-content.svelte";
    import EditorToolbar from "$lib/editor/editor-toolbar.svelte";
    import { beforeNavigate, goto } from "$app/navigation";
    import Input from "$lib/components/ui/input.svelte";
    import FieldGroup from "$lib/components/ui/field/field-group.svelte";
    import { createForm } from "@tanstack/svelte-form";
    import Field from "$lib/components/ui/field/field.svelte";
    import FieldLabel from "$lib/components/ui/field/field-label.svelte";
    import FieldError from "$lib/components/ui/field/field-error.svelte";
    import { createSystemPromptJsonSchema } from "$lib/api/prompts";
    import {
        createCreateSystemPromptMutation,
        invalidateSystemPromptsQueries,
    } from "$lib/query/prompts";
    import { toaster } from "$lib/components/toaster/store";
    import { isHTTPError } from "ky";
    import { getApiError } from "$lib/api/error";
    import Spinner from "$lib/components/ui/spinner.svelte";
    import Plus from "$lib/components/ui/icons/plus.svelte";

    /** @type {import('./$types').PageProps} */
    const props = $props();

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

    const createMutation = createCreateSystemPromptMutation();

    const form = createForm(() => ({
        defaultValues: {
            title: "",
            text: "",
        },
        validators: { onSubmit: createSystemPromptJsonSchema },
        onSubmit: async ({ value, formApi }) => {
            await createMutation.mutateAsync(
                [props.data.ky, { json: { text: value.text, title: value.title } }],
                {
                    onSuccess: async () => {
                        isDirty = false;
                        await goto("/prompts");
                        void invalidateSystemPromptsQueries(props.data.queryClient);
                    },
                    onError: (err) => {
                        let description = err.message;

                        if (isHTTPError(err)) {
                            const apiError = getApiError(err);
                            if (apiError) {
                                if (apiError.kind === "validation" && apiError.source) {
                                    formApi.setErrorMap({
                                        onSubmit: { fields: { [apiError.source]: apiError } },
                                    });
                                    return;
                                }
                                description = apiError.message;
                            }
                        }

                        toaster.push("Failed to create system prompt", {
                            description,
                            props: { variant: "destructive" },
                        });
                    },
                },
            );
        },
    }));
</script>

<svelte:window onbeforeunload={handleBeforeUnload} />

<form
    class="space-y-4"
    onsubmit={(e) => {
        e.preventDefault();
        form.setFieldValue("text", editorInstance.getMarkdown());
        void form.handleSubmit();
    }}
>
    <FieldGroup>
        <form.Field name="title">
            {#snippet children(field)}
                {@const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid}
                <Field data-invalid={isInvalid}>
                    <FieldLabel for={field.name}>Title</FieldLabel>
                    <Input
                        id={field.name}
                        name={field.name}
                        value={field.state.value}
                        onblur={field.handleBlur}
                        onchange={(e) => field.handleChange(e.currentTarget.value)}
                        aria-invalid={isInvalid}
                        placeholder="Title"
                        class="text-3xl! font-bold h-fit"
                        autocomplete="off"
                    />
                    {#if isInvalid}
                        <FieldError errors={field.state.meta.errors} />
                    {/if}
                </Field>
            {/snippet}
        </form.Field>
    </FieldGroup>

    <FieldGroup>
        <form.Field name="text">
            {#snippet children(field)}
                {@const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid}
                <Field data-invalid={isInvalid}>
                    <FieldLabel for={field.name}>Text</FieldLabel>

                    <Editor
                        options={{
                            content: "",
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
                            onUpdate: ({ editor }) => {
                                // It is possible that editor was not inited yet
                                if (initalDoc) {
                                    isDirty = !editor.state.doc.eq(initalDoc);
                                }
                            },
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
        <Action anchor variant="outline" href="/prompts">Cancel</Action>
        <Action button type="submit" disabled={!isDirty || form.state.isSubmitting}>
            {#if form.state.isSubmitting}
                <Spinner />
            {:else}
                <Plus />
            {/if}
            <span>Create</span>
        </Action>
    </StickyBar>
</form>
