<script module>
    const NUMBER_FORMAT = new Intl.NumberFormat("en-US");
</script>

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
            {@render li("Prompt", usage.prompt_tokens)}
            {@render li("Completion", usage.completion_tokens)}
        </ul>

        <div class="text-end text-xs text-muted-foreground">
            Total used: {NUMBER_FORMAT.format(usage.total_tokens)} tokens
        </div>

        <DialogFooter>
            <DialogContinueAction closeOnClick>Ok</DialogContinueAction>
        </DialogFooter>
    </DialogContent>
</Dialog>

{#snippet li(/** @type {string} */ label, /** @type {number} */ tokens)}
    {@const formatted = NUMBER_FORMAT.format(tokens)}
    <li class="flex items-baseline justify-between gap-2">
        <span class="text-muted-foreground">{label}</span>
        <Badge variant="outline">
            <code>
                {formatted}
            </code>
        </Badge>
    </li>
{/snippet}
