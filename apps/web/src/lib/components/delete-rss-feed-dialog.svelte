<script>
    import { getGlobalContext } from "$lib/global-context";
    import { createDeleteRssFeedMutation } from "$lib/query/rss-feeds";
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
     * @property {import('$lib/api/rss-feeds').RssFeedId} feedId
     * @property {() => Promise<void>} [onSuccess]
     */

    /** @type {Omit<import('$lib/components/ui/alert-dialog/alert-dialog.svelte').AlertDialogProps, 'children' | 'ref'> & Props} */
    const { feedId, onSuccess, ...props } = $props();
    const { ky, queryClient } = getGlobalContext();

    /** @type {HTMLDialogElement} */
    // svelte-ignore non_reactive_update
    let dialog;

    const deleteMutation = createDeleteRssFeedMutation();

    function deleteArticle() {
        deleteMutation.mutate([ky, { params: { id: feedId } }], {
            onError: (err) =>
                toaster.push("Failed to delete RSS feed", {
                    description: err.message,
                    props: { variant: "destructive" },
                }),
            onSuccess: async () => {
                await onSuccess?.();
                void queryClient.invalidateQueries({
                    queryKey: ["rss_feeds"],
                });
                void queryClient.invalidateQueries({
                    queryKey: ["articles"],
                });
                dialog.close();
            },
        });
    }
</script>

<AlertDialog bind:ref={dialog} {...props}>
    <AlertDialogContent size="sm">
        <AlertDialogHeader>
            <AlertDialogTitle>Delete RSS feed?</AlertDialogTitle>
            <AlertDialogDescription>
                This action will delete the RSS feed and it cannot be undone later.
            </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
            <AlertDialogCloseAction />
            <AlertDialogContinueAction
                disabled={deleteMutation.isPending}
                onclick={deleteArticle}
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
