<script>
    import { getGlobalContext } from "$lib/global-context";
    import { createProcessArticleMutation } from "$lib/query/articles";
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

    const processMutation = createProcessArticleMutation();

    const form = createForm(() => ({
        defaultValues: { context: "" },
        validators: {
            onSubmit: z.object({
                context: processArticleJsonSchema.shape.context.nonoptional(),
            }),
        },
        onSubmit: async ({ value, formApi }) => {
            await processMutation.mutateAsync(
                [ky, { params: { id: articleId }, json: { context: value.context || undefined } }],
                {
                    onError: (err) => {
                        if (isHTTPError(err)) {
                            const apiError = getApiError(err);
                            if (apiError) {
                                formApi.setErrorMap({
                                    // Set all API errors to the context field since it is the only field we have
                                    onSubmit: { fields: { context: apiError } },
                                });
                                return;
                            }
                        }
                        toaster.push("Failed to request article processing", {
                            description: err.message,
                            props: { variant: "destructive" },
                        });
                    },
                    onSuccess: () => {
                        void queryClient.invalidateQueries({
                            queryKey: ["articles"],
                        });
                        void queryClient.invalidateQueries({
                            queryKey: ["articles/stats"],
                        });
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
                <form.Field name="context">
                    {#snippet children(field)}
                        {@const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid}
                        <Field data-invalid={isInvalid}>
                            <FieldLabel>Additional context</FieldLabel>
                            <Textarea
                                id={field.name}
                                name={field.name}
                                value={field.state.value}
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
