<script>
    import { createForm } from "@tanstack/svelte-form";
    import Dialog from "$lib/components/ui/dialog/dialog.svelte";
    import DialogContent from "$lib/components/ui/dialog/dialog-content.svelte";
    import DialogX from "$lib/components/ui/dialog/dialog-x.svelte";
    import DialogHeader from "$lib/components/ui/dialog/dialog-header.svelte";
    import DialogTitle from "$lib/components/ui/dialog/dialog-title.svelte";
    import DialogDescription from "$lib/components/ui/dialog/dialog-description.svelte";
    import FieldGroup from "$lib/components/ui/field/field-group.svelte";
    import Field from "$lib/components/ui/field/field.svelte";
    import FieldLabel from "$lib/components/ui/field/field-label.svelte";
    import FieldError from "$lib/components/ui/field/field-error.svelte";
    import DialogFooter from "$lib/components/ui/dialog/dialog-footer.svelte";
    import DialogCloseAction from "$lib/components/ui/dialog/dialog-close-action.svelte";
    import DialogContinueAction from "$lib/components/ui/dialog/dialog-continue-action.svelte";
    import Spinner from "$lib/components/ui/spinner.svelte";
    import Input from "$lib/components/ui/input.svelte";
    import { createCreateRssFeedMutation } from "$lib/query/rss-feeds";
    import { createRssFeedJsonSchema } from "$lib/api/rss-feeds";
    import { getGlobalContext } from "$lib/global-context";
    import { isHTTPError } from "ky";
    import { getApiError } from "$lib/api/error";
    import { toaster } from "$lib/components/toaster/store";

    /** @type {Omit<import('$lib/components/ui/dialog/dialog.svelte').DialogProps, 'children' | 'ref'>} */
    let props = $props();
    const { ky, queryClient } = getGlobalContext();

    /** @type {HTMLDialogElement} */
    // svelte-ignore non_reactive_update
    let dialog;

    const createMutation = createCreateRssFeedMutation();

    const form = createForm(() => ({
        defaultValues: { url: "" },
        validators: {
            onSubmit: createRssFeedJsonSchema,
        },
        onSubmit: async ({ value, formApi }) => {
            await createMutation.mutateAsync([ky, { json: { url: value.url } }], {
                onError: (err) => {
                    if (isHTTPError(err)) {
                        const apiError = getApiError(err);
                        if (apiError) {
                            if (apiError.kind === "validation" && apiError.source) {
                                formApi.setErrorMap({
                                    onSubmit: { fields: { [apiError.source]: apiError } },
                                });
                            } else {
                                toaster.push("Failed to create RSS feed", {
                                    description: apiError.message,
                                    props: { variant: "destructive" },
                                });
                            }
                            return;
                        }
                    }
                    toaster.push("Failed to create RSS feed", {
                        description: err.message,
                        props: { variant: "destructive" },
                    });
                },
                onSuccess: () => {
                    void queryClient.invalidateQueries({
                        queryKey: ["rss_feeds"],
                    });
                    dialog.close();
                },
            });
        },
    }));
</script>

<Dialog bind:ref={dialog} {...props}>
    <DialogContent>
        <DialogX />
        <DialogHeader>
            <DialogTitle>New RSS feed</DialogTitle>
            <DialogDescription>Create RSS feed with configuration details below</DialogDescription>
        </DialogHeader>
        <form
            class="contents"
            onsubmit={(e) => {
                e.preventDefault();
                form.handleSubmit();
            }}
        >
            <FieldGroup>
                <form.Field name="url">
                    {#snippet children(field)}
                        {@const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid}
                        <Field data-invalid={isInvalid}>
                            <FieldLabel>Feed URL</FieldLabel>
                            <Input
                                id={field.name}
                                name={field.name}
                                value={field.state.value}
                                onblur={field.handleBlur}
                                onchange={(e) => field.handleChange(e.currentTarget.value)}
                                aria-invalid={isInvalid}
                                placeholder="https://feed/rss.xml"
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
                <FieldError errors={form.state.errors} />
            </FieldGroup>

            <DialogFooter>
                <DialogCloseAction onclick={() => form.reset()}>Cancel</DialogCloseAction>
                <DialogContinueAction type="submit" disabled={form.state.isSubmitting}>
                    {#if form.state.isSubmitting}
                        <Spinner />
                    {/if}
                    <span>Create</span>
                </DialogContinueAction>
            </DialogFooter>
        </form>
    </DialogContent>
</Dialog>
