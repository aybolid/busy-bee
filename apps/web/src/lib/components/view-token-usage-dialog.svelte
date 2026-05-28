<script module>
    const NUMBER_FORMAT = new Intl.NumberFormat("en-US");

    /**
     * @param {number | null | undefined} n
     * @returns {string}
     */
    function formatNumberOrUnknown(n) {
        return n ? NUMBER_FORMAT.format(n) : "Unknown";
    }
</script>

<script>
    import AccordionContent from "./ui/accordion/accordion-content.svelte";

    import AccordionHeader from "./ui/accordion/accordion-header.svelte";
    import AccordionItem from "./ui/accordion/accordion-item.svelte";
    import AccordionTrigger from "./ui/accordion/accordion-trigger.svelte";
    import Accordion from "./ui/accordion/accordion.svelte";
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
     * @property {import('$lib/api/article-processing-outputs').ArticleProcessingOutput} output
     */

    /** @type {Omit<import('$lib/components/ui/dialog/dialog.svelte').DialogProps, 'children' | 'ref'> & Props} */
    let { output, ...props } = $props();
</script>

<Dialog {...props}>
    <DialogContent>
        <DialogX />
        <DialogHeader>
            <DialogTitle>Token usage</DialogTitle>
            <DialogDescription>
                <code>{output.model}</code>
            </DialogDescription>
        </DialogHeader>

        <Accordion>
            <AccordionItem>
                <AccordionHeader>
                    <AccordionTrigger class="items-center">
                        <span>Prompt tokens</span>
                        <Badge variant="secondary">
                            <code>
                                {formatNumberOrUnknown(output.prompt_tokens)}
                            </code>
                        </Badge>
                    </AccordionTrigger>
                </AccordionHeader>
                <AccordionContent>
                    <ul class="flex flex-col gap-1">
                        {@render li("Cache creation", output.prompt_cache_creation_tokens)}
                        {@render li("Cached", output.prompt_cached_tokens)}
                        {@render li("Audio", output.prompt_audio_tokens)}
                    </ul>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionHeader>
                    <AccordionTrigger>
                        <span>Completion tokens</span>
                        <Badge variant="secondary">
                            <code>
                                {formatNumberOrUnknown(output.completion_tokens)}
                            </code>
                        </Badge>
                    </AccordionTrigger>
                </AccordionHeader>
                <AccordionContent>
                    <ul class="flex flex-col gap-1">
                        {@render li(
                            "Prediciton (accepted)",
                            output.completion_accepted_prediction_tokens,
                        )}
                        {@render li(
                            "Prediction (rejected)",
                            output.completion_rejected_prediction_tokens,
                        )}
                        {@render li("Reasoning", output.completion_reasoning_tokens)}
                        {@render li("Audio", output.completion_audio_tokens)}
                    </ul>
                </AccordionContent>
            </AccordionItem>
        </Accordion>

        <div class="text-end text-xs text-muted-foreground">
            Total used: {formatNumberOrUnknown(output.total_tokens)} tokens
        </div>

        <DialogFooter>
            <DialogContinueAction closeOnClick>Ok</DialogContinueAction>
        </DialogFooter>
    </DialogContent>
</Dialog>

{#snippet li(/** @type {string} */ label, /** @type {number | null | undefined} */ tokens)}
    {@const formatted = formatNumberOrUnknown(tokens)}
    <li class="flex items-baseline justify-between gap-2">
        <span class="text-muted-foreground">{label}</span>
        <Badge variant={formatted === "Unknown" ? "ghost" : "outline"}>
            <code>
                {formatted}
            </code>
        </Badge>
    </li>
{/snippet}
