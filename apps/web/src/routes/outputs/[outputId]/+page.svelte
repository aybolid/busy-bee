<script>
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import Badge from "$lib/components/ui/badge.svelte";
    import StickyBar from "$lib/components/ui/sticky-bar.svelte";
    import EllipsisVertical from "$lib/components/ui/icons/ellipsis-vertical.svelte";
    import MenuActionItem from "$lib/components/ui/menu/menu-action-item.svelte";
    import MenuContent from "$lib/components/ui/menu/menu-content.svelte";
    import MenuGroup from "$lib/components/ui/menu/menu-group.svelte";
    import MenuLabel from "$lib/components/ui/menu/menu-label.svelte";
    import Menu from "$lib/components/ui/menu/menu.svelte";
    import { createQuery } from "@tanstack/svelte-query";
    import dayjs from "dayjs";
    import { getArticleProcessingOutputQueryOptions } from "$lib/query/article-processing-outputs";
    import SvelteMarkdown from "@humanspeak/svelte-markdown";
    import ViewTokenUsageDialog from "$lib/components/view-token-usage-dialog.svelte";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    const outputId =
        /** @type {import('$lib/api/article-processing-outputs').ArticleProcessingOutputId} */ (
            $derived(props.params.outputId)
        );

    const output = createQuery(() =>
        getArticleProcessingOutputQueryOptions(props.data.ky, { params: { id: outputId } }),
    );
</script>

{#if output.isPending}
    <Pending />
{:else if output.isError}
    <ErrorAlert error={output.error} />
{:else if output.isSuccess}
    <article class="mx-auto prose max-w-4xl py-8 prose-app">
        <SvelteMarkdown source={output.data.output_text} />
    </article>

    <StickyBar>
        <div class="flex flex-wrap gap-2">
            <Badge variant="secondary">
                {dayjs(output.data.created_at).format("MMM DD, YYYY, HH:mm")}
            </Badge>
        </div>

        {@render menu(output.data)}
    </StickyBar>
{/if}

{#snippet menu(
    /** @type {import('$lib/api/article-processing-outputs').ArticleProcessingOutput} */ output,
)}
    <Menu>
        {#snippet trigger(props)}
            <Action button size="sm" variant="outline" {...props}>
                <EllipsisVertical />
                <span>Actions</span>
            </Action>
        {/snippet}
        <MenuContent>
            <MenuGroup>
                <MenuLabel>Output actions</MenuLabel>
                <MenuActionItem anchor href="/outputs/{output.id}/edit">Edit</MenuActionItem>
                <ViewTokenUsageDialog usage={output.usage} model={output.model}>
                    {#snippet trigger(props)}
                        <MenuActionItem button keepOpen {...props}>Usage</MenuActionItem>
                    {/snippet}
                </ViewTokenUsageDialog>
                {#if output.article_id}
                    <MenuActionItem anchor href="/articles/{output.article_id}">
                        View article
                    </MenuActionItem>
                {/if}
            </MenuGroup>
        </MenuContent>
    </Menu>
{/snippet}
