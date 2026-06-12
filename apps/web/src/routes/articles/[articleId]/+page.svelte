<script>
    import ArticleStatus from "$lib/components/article-status.svelte";
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import Badge from "$lib/components/ui/badge.svelte";
    import StickyBar from "$lib/components/ui/sticky-bar.svelte";
    import EllipsisVertical from "$lib/components/ui/icons/ellipsis-vertical.svelte";
    import { getArticleQueryOptions } from "$lib/query/articles";
    import { createQuery } from "@tanstack/svelte-query";
    import dayjs from "dayjs";
    import Popover from "$lib/components/ui/popover/popover.svelte";
    import PopoverContent from "$lib/components/ui/popover/popover-content.svelte";
    import ArticleActionsMenu from "$lib/components/article-actions-menu.svelte";
    import { goto } from "$app/navigation";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    const articleId = /** @type {import('$lib/api/articles').ArticleId} */ (
        $derived(props.params.articleId)
    );

    const article = createQuery(() =>
        getArticleQueryOptions(props.data.ky, { params: { id: articleId } }),
    );
</script>

{#if article.isPending}
    <Pending />
{:else if article.isError}
    <ErrorAlert error={article.error} />
{:else if article.isSuccess}
    <article class="mx-auto prose max-w-4xl py-8 prose-app">
        {#if article.data.favicon}
            <div class="size-8 pb-16">
                <img src={article.data.favicon} alt="favicon" />
            </div>
        {/if}
        <h1>{article.data.title}</h1>
        <hr />
        {@html article.data.content}
    </article>

    <StickyBar>
        <div class="flex flex-wrap gap-2">
            {#if article.data.byline}
                <Badge>{article.data.byline}</Badge>
            {/if}
            {#if article.data.published_time}
                <Badge variant="secondary">
                    {dayjs(article.data.published_time).format("MMM DD, YYYY, HH:mm")}
                </Badge>
            {/if}

            {#if article.data.status === "error"}
                <Popover position="top">
                    {#snippet trigger(props)}
                        <button {...props}>
                            <ArticleStatus status="error" />
                        </button>
                    {/snippet}
                    <PopoverContent class="w-72">
                        <ErrorAlert
                            title="Processing error"
                            description={article.data.error_reason}
                        />
                    </PopoverContent>
                </Popover>
            {:else}
                <ArticleStatus status={article.data.status} />
            {/if}
        </div>

        <ArticleActionsMenu article={article.data} withoutView onDelete={() => goto("/articles")}>
            {#snippet trigger(props)}
                <Action button size="sm" variant="outline" {...props}>
                    <EllipsisVertical />
                    <span>Actions</span>
                </Action>
            {/snippet}
        </ArticleActionsMenu>
    </StickyBar>
{/if}
