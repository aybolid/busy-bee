<script>
    import { getGlobalContext } from "$lib/global-context";
    import { createForm } from "@tanstack/svelte-form";
    import DialogCloseAction from "./ui/dialog/dialog-close-action.svelte";
    import DialogContent from "./ui/dialog/dialog-content.svelte";
    import DialogContinueAction from "./ui/dialog/dialog-continue-action.svelte";
    import DialogDescription from "./ui/dialog/dialog-description.svelte";
    import DialogFooter from "./ui/dialog/dialog-footer.svelte";
    import DialogHeader from "./ui/dialog/dialog-header.svelte";
    import DialogTitle from "./ui/dialog/dialog-title.svelte";
    import DialogX from "./ui/dialog/dialog-x.svelte";
    import Dialog from "./ui/dialog/dialog.svelte";
    import FieldError from "./ui/field/field-error.svelte";
    import FieldLabel from "./ui/field/field-label.svelte";
    import Field from "./ui/field/field.svelte";
    import Spinner from "./ui/spinner.svelte";
    import FieldGroup from "./ui/field/field-group.svelte";
    import { isHTTPError } from "ky";
    import { getApiError } from "$lib/api/error";
    import { toaster } from "./toaster/store";
    import {
        createUpdateInstructionPromptMutation,
        invalidateInstructionPromptsQueries,
    } from "$lib/query/prompts";
    import { updateInstructionPromptJsonSchema } from "$lib/api/prompts";
    import InputGroup from "./ui/input-group/input-group.svelte";
    import InputGroupTextarea from "./ui/input-group/input-group-textarea.svelte";
    import InputGroupAddon from "./ui/input-group/input-group-addon.svelte";
    import InputGroupText from "./ui/input-group/input-group-text.svelte";
    import { getMaxLengthConstraint } from "$lib/schema-utils";
    import Plus from "./ui/icons/plus.svelte";
    import Input from "./ui/input.svelte";
    import z from "zod";

    /** @type {Omit<import('$lib/components/ui/dialog/dialog.svelte').DialogProps, 'children' | 'ref'> & { instructionPrompt: import('$lib/api/prompts').InstructionPrompt }} */
    let { instructionPrompt, ...props } = $props();
    const { ky, queryClient } = getGlobalContext();

    /** @type {HTMLDialogElement} */
    // svelte-ignore non_reactive_update
    let dialog;

    const updateMutation = createUpdateInstructionPromptMutation();

    const form = createForm(() => ({
        defaultValues: {
            title: instructionPrompt.title,
            text: instructionPrompt.text,
        },
        validators: {
            onSubmit: z.object({
                title: updateInstructionPromptJsonSchema.shape.title.nonoptional(),
                text: updateInstructionPromptJsonSchema.shape.text.nonoptional(),
            }),
        },
        onSubmit: async ({ value, formApi }) => {
            await updateMutation.mutateAsync(
                [
                    ky,
                    {
                        params: { id: instructionPrompt.id },
                        json: {
                            version: instructionPrompt.version,
                            title: value.title,
                            text: value.text,
                        },
                    },
                ],
                {
                    onError: (err) => {
                        let description = err.message;

                        if (isHTTPError(err)) {
                            const apiError = getApiError(err);
                            if (apiError) {
                                if (apiError.kind === "validation" && apiError.source) {
                                    if (apiError.source === "version") {
                                        toaster.push("Failed to update instruction prompt", {
                                            description: "Prompt version mismatch",
                                            props: { variant: "destructive" },
                                        });
                                    } else {
                                        formApi.setErrorMap({
                                            onSubmit: { fields: { [apiError.source]: apiError } },
                                        });
                                    }
                                    return;
                                }
                                description = apiError.message;
                            }
                        }

                        toaster.push("Failed to update instruction prompt", {
                            description,
                            props: { variant: "destructive" },
                        });
                    },
                    onSuccess: () => {
                        void invalidateInstructionPromptsQueries(queryClient);
                        dialog.close();
                    },
                },
            );
        },
    }));
</script>

<Dialog bind:ref={dialog} {...props}>
    <DialogContent>
        <DialogX />
        <DialogHeader>
            <DialogTitle>Edit instruction prompt</DialogTitle>
            <DialogDescription>Update existing instruction prompt.</DialogDescription>
        </DialogHeader>
        <form
            class="contents"
            onsubmit={(e) => {
                e.preventDefault();
                form.handleSubmit();
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
                            <InputGroup>
                                <InputGroupTextarea
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onblur={field.handleBlur}
                                    oninput={(e) => field.handleChange(e.currentTarget.value)}
                                    aria-invalid={isInvalid}
                                    placeholder="Do this and not that..."
                                    autocomplete="off"
                                    rows={4}
                                />
                                <InputGroupAddon align="block-end">
                                    <InputGroupText class="text-xs tabular-nums">
                                        {field.state.value.trim().length}/{getMaxLengthConstraint(
                                            updateInstructionPromptJsonSchema.shape.text,
                                        )}
                                    </InputGroupText>
                                </InputGroupAddon>
                            </InputGroup>
                            {#if isInvalid}
                                <FieldError errors={field.state.meta.errors} />
                            {/if}
                        </Field>
                    {/snippet}
                </form.Field>
            </FieldGroup>

            <DialogFooter>
                <DialogCloseAction onclick={() => form.reset()}>Cancel</DialogCloseAction>
                <DialogContinueAction type="submit" disabled={form.state.isSubmitting}>
                    {#if form.state.isSubmitting}
                        <Spinner />
                    {/if}
                    <span>Save</span>
                </DialogContinueAction>
            </DialogFooter>
        </form>
    </DialogContent>
</Dialog>
