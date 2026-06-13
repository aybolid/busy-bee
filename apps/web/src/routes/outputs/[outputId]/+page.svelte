<script>
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import Badge from "$lib/components/ui/badge.svelte";
    import StickyBar from "$lib/components/ui/sticky-bar.svelte";
    import EllipsisVertical from "$lib/components/ui/icons/ellipsis-vertical.svelte";
    import { createQuery } from "@tanstack/svelte-query";
    import { getOutputQueryOptions } from "$lib/query/outputs";
    import SvelteMarkdown from "@humanspeak/svelte-markdown";
    import OutputActionsMenu from "$lib/components/output-actions-menu.svelte";
    import { goto } from "$app/navigation";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    const outputId = /** @type {import('$lib/api/outputs').OutputId} */ (
        $derived(props.params.outputId)
    );

    const output = createQuery(() =>
        getOutputQueryOptions(props.data.ky, { params: { id: outputId } }),
    );
</script>

{#if output.isPending}
    <Pending />
{:else if output.isError}
    <ErrorAlert error={output.error} />
{:else if output.isSuccess}
    <article class="mx-auto prose max-w-4xl py-8 prose-app">
        <SvelteMarkdown source={output.data.text} />
    </article>

    <StickyBar>
        <div class="flex flex-wrap gap-2">
            <Badge variant="secondary">
                {output.data.formattedCreatedAt()}
            </Badge>
        </div>

        <OutputActionsMenu output={output.data} withoutView onDelete={() => goto("/outputs")}>
            {#snippet trigger(props)}
                <Action button size="sm" variant="outline" {...props}>
                    <EllipsisVertical />
                    <span>Actions</span>
                </Action>
            {/snippet}
        </OutputActionsMenu>
    </StickyBar>
{/if}
