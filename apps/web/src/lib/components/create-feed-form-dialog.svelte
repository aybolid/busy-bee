<script module>
    const INTERVAL_OPTIONS =
        /** @type {const} */
        ([
            { label: "5 mins", seconds: 300 },
            { label: "10 mins", seconds: 600 },
            { label: "15 mins", seconds: 900 },
            { label: "1 hour", seconds: 3600 },
            { label: "2 hours", seconds: 7200 },
            { label: "4 hours", seconds: 14400 },
            { label: "8 hours", seconds: 28800 },
            { label: "16 hours", seconds: 57600 },
            { label: "1 day", seconds: 86400 },
            { label: "3 days", seconds: 259200 },
            { label: "7 days", seconds: 604800 },
        ]);
</script>

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
    import FieldDescription from "$lib/components/ui/field/field-description.svelte";
    import NativeSelect from "$lib/components/ui/native-select/native-select.svelte";
    import NativeSelectOption from "$lib/components/ui/native-select/native-select-option.svelte";

    /** @type {Omit<import('$lib/components/ui/dialog/dialog.svelte').DialogProps, 'children' | 'ref'> & { defaultUrl?: string }} */
    let { defaultUrl = "", ...props } = $props();
    const { ky, queryClient } = getGlobalContext();

    /** @type {HTMLDialogElement} */
    // svelte-ignore non_reactive_update
    let dialog;

    const createMutation = createCreateRssFeedMutation();

    const form = createForm(() => ({
        defaultValues: {
            url: defaultUrl,
            max_concurrent_requests: 5,
            fetch_interval_seconds: /** @type {number} */ (INTERVAL_OPTIONS[2].seconds),
        },
        validators: {
            onSubmit: createRssFeedJsonSchema,
        },
        onSubmit: async ({ value, formApi }) => {
            await createMutation.mutateAsync(
                [
                    ky,
                    {
                        json: {
                            url: value.url,
                            max_concurrent_requests: value.max_concurrent_requests,
                            fetch_interval_seconds: value.fetch_interval_seconds,
                        },
                    },
                ],
                {
                    onError: (err) => {
                        if (isHTTPError(err)) {
                            const apiError = getApiError(err);
                            if (apiError) {
                                if (
                                    apiError.kind === "validation" &&
                                    apiError.source &&
                                    apiError.source !== "."
                                ) {
                                    formApi.setErrorMap({
                                        onSubmit: { fields: { [apiError.source]: apiError } },
                                    });
                                } else {
                                    toaster.push("Failed to create RSS feed", {
                                        description: apiError.message,
                                        props: { variant: "destructive" },
                                    });
                                }
                            }
                        } else {
                            toaster.push("Failed to create RSS feed", {
                                description: err.message,
                                props: { variant: "destructive" },
                            });
                        }
                    },
                    onSuccess: () => {
                        void queryClient.invalidateQueries({
                            queryKey: ["rss_feeds"],
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
            <DialogTitle>New RSS feed</DialogTitle>
            <DialogDescription>Create RSS feed with configuration details below.</DialogDescription>
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
                            <FieldLabel for={field.name}>Feed URL</FieldLabel>
                            <Input
                                disabled={!!defaultUrl}
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
                <form.Field name="max_concurrent_requests">
                    {#snippet children(field)}
                        {@const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid}
                        <Field data-invalid={isInvalid}>
                            <FieldLabel for={field.name}>Max requests</FieldLabel>
                            <Input
                                type="number"
                                step="1"
                                min="1"
                                max="255"
                                id={field.name}
                                name={field.name}
                                value={field.state.value}
                                onblur={field.handleBlur}
                                onchange={(e) => field.handleChange(e.currentTarget.valueAsNumber)}
                                aria-invalid={isInvalid}
                                placeholder="5"
                                autocomplete="off"
                            />
                            <FieldDescription>
                                Maximum number of concurrent requests to make when fetching the feed
                                articles.
                            </FieldDescription>
                            {#if isInvalid}
                                <FieldError errors={field.state.meta.errors} />
                            {/if}
                        </Field>
                    {/snippet}
                </form.Field>
            </FieldGroup>

            <FieldGroup>
                <form.Field name="fetch_interval_seconds">
                    {#snippet children(field)}
                        {@const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid}
                        <Field data-invalid={isInvalid}>
                            <FieldLabel for={field.name}>Fetch interval</FieldLabel>
                            <NativeSelect
                                id={field.name}
                                name={field.name}
                                value={field.state.value}
                                onblur={field.handleBlur}
                                aria-invalid={isInvalid}
                                onchange={(e) =>
                                    field.handleChange(parseInt(e.currentTarget.value, 10))}
                            >
                                <NativeSelectOption disabled>Select interval</NativeSelectOption>
                                {#each INTERVAL_OPTIONS as option}
                                    <NativeSelectOption value={option.seconds}>
                                        {option.label}
                                    </NativeSelectOption>
                                {/each}
                            </NativeSelect>
                            <FieldDescription>Interval between fetching the feed.</FieldDescription>
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
                    <span>Create</span>
                </DialogContinueAction>
            </DialogFooter>
        </form>
    </DialogContent>
</Dialog>
