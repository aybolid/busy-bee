<script>
    import Badge from "./ui/badge.svelte";
    import DialogContent from "./ui/dialog/dialog-content.svelte";
    import DialogContinueAction from "./ui/dialog/dialog-continue-action.svelte";
    import DialogDescription from "./ui/dialog/dialog-description.svelte";
    import DialogFooter from "./ui/dialog/dialog-footer.svelte";
    import DialogHeader from "./ui/dialog/dialog-header.svelte";
    import DialogTitle from "./ui/dialog/dialog-title.svelte";
    import DialogX from "./ui/dialog/dialog-x.svelte";
    import Dialog from "./ui/dialog/dialog.svelte";

    /**
     * @typedef {Object} Props
     * @property {import('$lib/api/outputs').Usage} usage
     * @property {string} model
     */

    /** @type {Omit<import('$lib/components/ui/dialog/dialog.svelte').DialogProps, 'children' | 'ref'> & Props} */
    let { usage, model, ...props } = $props();
</script>

<Dialog {...props}>
    <DialogContent>
        <DialogX />
        <DialogHeader>
            <DialogTitle>Token usage</DialogTitle>
            <DialogDescription>
                <code>{model}</code>
            </DialogDescription>
        </DialogHeader>

        <ul class="flex flex-col gap-1">
            {@render li("Prompt", usage.formattedPromptTokens())}
            {@render li("Completion", usage.formattedCompletionTokens())}
        </ul>

        <div class="text-end text-xs text-muted-foreground">
            Total used: {usage.formattedTotalTokens()} tokens
        </div>

        <DialogFooter>
            <DialogContinueAction closeOnClick>Ok</DialogContinueAction>
        </DialogFooter>
    </DialogContent>
</Dialog>

{#snippet li(/** @type {string} */ label, /** @type {string} */ formattedTokens)}
    <li class="flex items-baseline justify-between gap-2">
        <span class="text-muted-foreground">{label}</span>
        <Badge variant="outline">
            <code>
                {formattedTokens}
            </code>
        </Badge>
    </li>
{/snippet}
