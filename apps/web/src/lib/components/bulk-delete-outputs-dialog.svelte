<script>
    import { getGlobalContext } from "$lib/global-context";
    import { createBulkDeleteOutputsMutation, invalidateOutputsQueries } from "$lib/query/outputs";
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
     * @property {Array<import('$lib/api/outputs').OutputId>} outputIds
     * @property {() => Promise<void> | void} [onSuccess]
     */

    /** @type {Omit<import('$lib/components/ui/alert-dialog/alert-dialog.svelte').AlertDialogProps, 'children'> & Props} */
    const { outputIds, onSuccess, ...props } = $props();
    const { ky, queryClient } = getGlobalContext();

    /** @type {HTMLDialogElement} */
    // svelte-ignore non_reactive_update
    let dialog;

    const deleteMutation = createBulkDeleteOutputsMutation();

    function deleteOutputs() {
        deleteMutation.mutate([ky, { json: { ids: outputIds } }], {
            onError: (err) =>
                toaster.push("Failed to delete outputs", {
                    description: err.message,
                    props: { variant: "destructive" },
                }),
            onSuccess: async () => {
                await onSuccess?.();
                void invalidateOutputsQueries(queryClient);
                dialog.close();
            },
        });
    }
</script>

<AlertDialog bind:ref={dialog} {...props}>
    <AlertDialogContent size="sm">
        <AlertDialogHeader>
            <AlertDialogTitle>Delete {outputIds.length} output(s)?</AlertDialogTitle>
            <AlertDialogDescription>
                This action will delete outputs and it cannot be undone later.
            </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
            <AlertDialogCloseAction />
            <AlertDialogContinueAction
                disabled={deleteMutation.isPending}
                onclick={deleteOutputs}
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
