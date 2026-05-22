<script>
    import { getGlobalContext } from "$lib/global-context";
    import { createDeleteArticleMutation } from "$lib/query/articles";
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
     * @property {import('$lib/api/articles').ArticleId} articleId
     * @property {() => Promise<void>} [onSuccess]
     */

    /** @type {Omit<import('$lib/components/ui/alert-dialog/alert-dialog.svelte').AlertDialogProps, 'children' | 'ref'> & Props} */
    const { articleId, onSuccess, ...props } = $props();
    const { ky, queryClient } = getGlobalContext();

    /** @type {HTMLDialogElement} */
    // svelte-ignore non_reactive_update
    let dialog;

    const deleteMutation = createDeleteArticleMutation();

    function deleteArticle() {
        deleteMutation.mutate([ky, { params: { id: articleId } }], {
            onError: (err) => alert(err.message),
            onSuccess: async () => {
                await onSuccess?.();
                void queryClient.invalidateQueries({
                    predicate: (q) =>
                        q.queryKey[0] === "articles" || q.queryKey[0] === "articles/stats",
                });
                dialog.close();
            },
        });
    }
</script>

<AlertDialog bind:ref={dialog} {...props}>
    <AlertDialogContent size="sm">
        <AlertDialogHeader>
            <AlertDialogTitle>Delete article?</AlertDialogTitle>
            <AlertDialogDescription>
                This action will delete the article and it cannot be undone later.
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
