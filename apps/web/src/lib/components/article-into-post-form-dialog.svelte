<script>
    import { getGlobalContext } from "$lib/global-context";
    import { createProcessArticleMutation } from "$lib/query/articles";
    import DialogCloseAction from "./ui/dialog/dialog-close-action.svelte";
    import DialogContent from "./ui/dialog/dialog-content.svelte";
    import DialogContinueAction from "./ui/dialog/dialog-continue-action.svelte";
    import DialogDescription from "./ui/dialog/dialog-description.svelte";
    import DialogFooter from "./ui/dialog/dialog-footer.svelte";
    import DialogHeader from "./ui/dialog/dialog-header.svelte";
    import DialogTitle from "./ui/dialog/dialog-title.svelte";
    import DialogX from "./ui/dialog/dialog-x.svelte";
    import Dialog from "./ui/dialog/dialog.svelte";
    import Spinner from "./ui/spinner.svelte";

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

    function articleIntoPost() {
        processMutation.mutate([ky, { params: { id: articleId } }], {
            onError: (err) => alert(err.message),
            onSuccess: () => {
                void queryClient.invalidateQueries({
                    predicate: (q) =>
                        q.queryKey[0] === "articles" || q.queryKey[0] === "articles/stats",
                });
                dialog.close();
            },
        });
    }
</script>

<Dialog bind:ref={dialog} {...props}>
    <DialogContent>
        <DialogX />
        <DialogHeader>
            <DialogTitle>Convert into post</DialogTitle>
            <DialogDescription>Create a post from the article</DialogDescription>
        </DialogHeader>
        <form
            class="contents"
            onsubmit={(e) => {
                e.preventDefault();
                articleIntoPost();
            }}
        >
            <DialogFooter>
                <DialogCloseAction>Cancel</DialogCloseAction>
                <DialogContinueAction type="submit" disabled={processMutation.isPending}>
                    {#if processMutation.isPending}
                        <Spinner />
                    {/if}
                    <span>Convert</span>
                </DialogContinueAction>
            </DialogFooter>
        </form>
    </DialogContent>
</Dialog>
