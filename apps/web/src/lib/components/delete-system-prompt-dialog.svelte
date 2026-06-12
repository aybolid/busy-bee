<script>
    import { getGlobalContext } from "$lib/global-context";
    import {
        createDeleteSystemPromptMutation,
        invalidateSystemPromptsQueries,
    } from "$lib/query/prompts";
    import { toaster } from "./toaster/store";
    import AlertDialogCloseAction from "./ui/alert-dialog/alert-dialog-close-action.svelte";
    import AlertDialogContent from "./ui/alert-dialog/alert-dialog-content.svelte";
    import AlertDialogContinueAction from "./ui/alert-dialog/alert-dialog-continue-action.svelte";
    import AlertDialogDescription from "./ui/alert-dialog/alert-dialog-description.svelte";
    import AlertDialogFooter from "./ui/alert-dialog/alert-dialog-footer.svelte";
    import AlertDialogHeader from "./ui/alert-dialog/alert-dialog-header.svelte";
    import AlertDialogTitle from "./ui/alert-dialog/alert-dialog-title.svelte";
    import AlertDialog from "./ui/alert-dialog/alert-dialog.svelte";
    import Trash from "./ui/icons/trash.svelte";
    import Spinner from "./ui/spinner.svelte";

    /**
     * @typedef {Object} Props
     * @property {import('$lib/api/prompts').SystemPromptId} systemPromptId
     * @property {() => Promise<void> | void} [onSuccess]
     */

    /** @type {Omit<import('$lib/components/ui/alert-dialog/alert-dialog.svelte').AlertDialogProps, 'children' | 'ref'> & Props} */
    const { systemPromptId, onSuccess, ...props } = $props();
    const { ky, queryClient } = getGlobalContext();

    /** @type {HTMLDialogElement} */
    // svelte-ignore non_reactive_update
    let dialog;

    const deleteMutation = createDeleteSystemPromptMutation();

    function deleteSystemPrompt() {
        deleteMutation.mutate([ky, { params: { id: systemPromptId } }], {
            onError: (err) =>
                toaster.push("Failed to delete system prompt", {
                    description: err.message,
                    props: { variant: "destructive" },
                }),
            onSuccess: async () => {
                await onSuccess?.();
                void invalidateSystemPromptsQueries(queryClient);
                dialog.close();
            },
        });
    }
</script>

<AlertDialog bind:ref={dialog} {...props}>
    <AlertDialogContent size="sm">
        <AlertDialogHeader>
            <AlertDialogTitle>Delete system prompt?</AlertDialogTitle>
            <AlertDialogDescription>
                This action will delete the system prompt. It cannot be undone later.
            </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
            <AlertDialogCloseAction />
            <AlertDialogContinueAction
                disabled={deleteMutation.isPending}
                onclick={deleteSystemPrompt}
                variant="destructive"
            >
                {#if deleteMutation.isPending}
                    <Spinner />
                {:else}
                    <Trash />
                {/if}
                <span>Delete</span>
            </AlertDialogContinueAction>
        </AlertDialogFooter>
    </AlertDialogContent>
</AlertDialog>
