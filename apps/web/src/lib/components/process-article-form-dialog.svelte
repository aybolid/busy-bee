<script>
    import { getGlobalContext } from "$lib/global-context";
    import {
        createProcessArticleMutation,
        invalidateArticlesQueries,
        invalidateArticleStatsQueries,
    } from "$lib/query/articles";
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
    import FieldDescription from "./ui/field/field-description.svelte";
    import FieldError from "./ui/field/field-error.svelte";
    import FieldLabel from "./ui/field/field-label.svelte";
    import Field from "./ui/field/field.svelte";
    import Spinner from "./ui/spinner.svelte";
    import Textarea from "./ui/textarea.svelte";
    import FieldGroup from "./ui/field/field-group.svelte";
    import z from "zod";
    import { processArticleJsonSchema } from "$lib/api/articles";
    import { isHTTPError } from "ky";
    import { getApiError } from "$lib/api/error";
    import { toaster } from "./toaster/store";
    import { createQuery } from "@tanstack/svelte-query";
    import {
        getInstructionPromptsQueryOptions,
        getSystemPromptsQueryOptions,
    } from "$lib/query/prompts";
    import NativeSelect from "./ui/native-select/native-select.svelte";
    import NativeSelectOption from "./ui/native-select/native-select-option.svelte";
    import ErrorAlert from "./error-alert.svelte";
    import Badge from "./ui/badge.svelte";
    import X from "./ui/icons/x.svelte";
    import { instructionPromptIdSchema } from "$lib/api/prompts";
    import { slide } from "svelte/transition";

    /**
     * @typedef {Object} FormProps
     * @property {import('$lib/api/articles').ArticleId} articleId
     */

    /** @type {Omit<import('$lib/components/ui/dialog/dialog.svelte').DialogProps, 'children' | 'ref'> & FormProps} */
    let { articleId, ...props } = $props();
    const { ky, queryClient } = getGlobalContext();

    /** @type {HTMLDialogElement} */
    // svelte-ignore non_reactive_update
    let dialog;
    let open = $state(false);

    $effect(() => {
        const observer = new MutationObserver((mutations) => {
            for (const mutation of mutations) {
                if (mutation.attributeName === "open") {
                    open = dialog.open;
                }
            }
        });

        observer.observe(dialog, { attributes: true });

        return () => observer.disconnect();
    });

    const systemPrompts = createQuery(() => ({
        ...getSystemPromptsQueryOptions(ky),
        enabled: open,
    }));
    const instructionPrompts = createQuery(() => ({
        ...getInstructionPromptsQueryOptions(ky),
        enabled: open,
    }));

    const processMutation = createProcessArticleMutation();

    const form = createForm(() => ({
        defaultValues: {
            // FIXME: any types
            system_prompt_id: /** @type {*} */ (""),
            instruction_ids: /** @type {Array<*>} */ ([]),
            context: "",
        },
        validators: {
            onSubmit: z.object({
                system_prompt_id: processArticleJsonSchema.shape.system_prompt_id,
                instruction_ids: z.array(instructionPromptIdSchema).max(255),
                context: processArticleJsonSchema.shape.context.nonoptional(),
            }),
        },
        onSubmit: async ({ value, formApi }) => {
            await processMutation.mutateAsync(
                [
                    ky,
                    {
                        params: { id: articleId },
                        json: {
                            system_prompt_id: value.system_prompt_id,
                            instruction_ids: value.instruction_ids?.length
                                ? value.instruction_ids
                                : undefined,
                            context: value.context || undefined,
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
                                    formApi.setErrorMap({
                                        onSubmit: { fields: { [apiError.source]: apiError } },
                                    });
                                    return;
                                }

                                description = apiError.message;
                            }
                        }

                        toaster.push("Failed to request article processing", {
                            description,
                            props: { variant: "destructive" },
                        });
                    },
                    onSuccess: () => {
                        void invalidateArticlesQueries(queryClient);
                        void invalidateArticleStatsQueries(queryClient);
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
            <DialogTitle>Process</DialogTitle>
            <DialogDescription>Schedule article for being processed by LLM</DialogDescription>
        </DialogHeader>
        <form
            class="contents"
            onsubmit={(e) => {
                e.preventDefault();
                form.handleSubmit();
            }}
        >
            <FieldGroup>
                <form.Field name="system_prompt_id">
                    {#snippet children(field)}
                        {@const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid}

                        <Field data-invalid={isInvalid}>
                            <FieldLabel for={field.name}>System prompt</FieldLabel>
                            {#if systemPrompts.isError}
                                <ErrorAlert
                                    title="Failed to load system prompts"
                                    description={systemPrompts.error.message}
                                />
                            {:else}
                                <NativeSelect
                                    disabled={systemPrompts.isPending}
                                    id={field.name}
                                    name={field.name}
                                    onblur={field.handleBlur}
                                    value={field.state.value}
                                    onchange={(e) => {
                                        const value = e.currentTarget.value;
                                        if (!value) return;
                                        field.handleChange(value);
                                    }}
                                    aria-invalid={isInvalid}
                                >
                                    <NativeSelectOption value="" disabled>
                                        Select system prompt
                                    </NativeSelectOption>
                                    {#if systemPrompts.isSuccess}
                                        {#each systemPrompts.data as prompt}
                                            <NativeSelectOption value={prompt.id}>
                                                {prompt.title}
                                            </NativeSelectOption>
                                        {/each}
                                    {/if}
                                </NativeSelect>
                            {/if}
                            {#if isInvalid}
                                <FieldError errors={field.state.meta.errors} />
                            {/if}
                        </Field>
                    {/snippet}
                </form.Field>
            </FieldGroup>

            <FieldGroup>
                <form.Field name="instruction_ids">
                    {#snippet children(field)}
                        {@const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid}

                        <Field data-invalid={isInvalid}>
                            <FieldLabel for={field.name}>Instructions</FieldLabel>

                            {#if field.state.value.length !== 0}
                                <div
                                    class="flex gap-1 flex-wrap"
                                    transition:slide={{ duration: 150 }}
                                >
                                    {#each field.state.value as instructionPromptId}
                                        {@const prompt = instructionPrompts.data?.find(
                                            (p) => p.id === instructionPromptId,
                                        )}
                                        {#if prompt}
                                            <Badge
                                                variant="secondary"
                                                role="button"
                                                onclick={() =>
                                                    field.handleChange((prev) =>
                                                        prev.filter(
                                                            (id) => id !== instructionPromptId,
                                                        ),
                                                    )}
                                                class="cursor-default select-none"
                                            >
                                                <span class="max-w-32 truncate">
                                                    {prompt.title}
                                                </span>
                                                <X />
                                            </Badge>
                                        {/if}
                                    {/each}
                                </div>
                            {/if}

                            {#if instructionPrompts.isError}
                                <ErrorAlert
                                    title="Failed to load instruction prompts"
                                    description={instructionPrompts.error.message}
                                />
                            {:else}
                                <NativeSelect
                                    disabled={instructionPrompts.isPending}
                                    id={field.name}
                                    name={field.name}
                                    value=""
                                    onblur={field.handleBlur}
                                    onchange={(e) => {
                                        const value = e.currentTarget.value;
                                        if (!value) return;
                                        field.handleChange((prev) => [...prev, value]);
                                    }}
                                    aria-invalid={isInvalid}
                                >
                                    <NativeSelectOption value="" disabled>
                                        Select instruction
                                    </NativeSelectOption>
                                    {#if instructionPrompts.isSuccess}
                                        {#each instructionPrompts.data.filter((p) => !field.state.value.includes(p.id)) as prompt}
                                            <NativeSelectOption value={prompt.id}>
                                                {prompt.title}
                                            </NativeSelectOption>
                                        {/each}
                                    {/if}
                                </NativeSelect>
                            {/if}

                            {#if isInvalid}
                                <FieldError errors={field.state.meta.errors} />
                            {/if}
                        </Field>
                    {/snippet}
                </form.Field>
            </FieldGroup>

            <FieldGroup>
                <form.Field name="context">
                    {#snippet children(field)}
                        {@const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid}
                        <Field data-invalid={isInvalid}>
                            <FieldLabel for={field.name}>Additional context</FieldLabel>
                            <Textarea
                                id={field.name}
                                name={field.name}
                                onblur={field.handleBlur}
                                onchange={(e) => field.handleChange(e.currentTarget.value)}
                                aria-invalid={isInvalid}
                                placeholder="Do this and not that..."
                                autocomplete="off"
                                rows={4}
                            />
                            <FieldDescription>
                                Provide an additional context for LLM
                            </FieldDescription>
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
                    <span>Process</span>
                </DialogContinueAction>
            </DialogFooter>
        </form>
    </DialogContent>
</Dialog>
