<script>
    import { goto } from "$app/navigation";
    import ArticleStatus from "$lib/components/article-status.svelte";
    import DeleteArticleAlertDialog from "$lib/components/delete-article-alert-dialog.svelte";
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import Badge from "$lib/components/ui/badge.svelte";
    import StickyBottomBar from "$lib/components/ui/sticky-bottom-bar.svelte";
    import EllipsisVertical from "$lib/components/ui/icons/ellipsis-vertical.svelte";
    import ExternalLink from "$lib/components/ui/icons/external-link.svelte";
    import Trash from "$lib/components/ui/icons/trash.svelte";
    import MenuActionItem from "$lib/components/ui/menu/menu-action-item.svelte";
    import MenuContent from "$lib/components/ui/menu/menu-content.svelte";
    import MenuGroup from "$lib/components/ui/menu/menu-group.svelte";
    import MenuLabel from "$lib/components/ui/menu/menu-label.svelte";
    import Menu from "$lib/components/ui/menu/menu.svelte";
    import { getArticleQueryOptions } from "$lib/query/articles";
    import { createQuery } from "@tanstack/svelte-query";
    import dayjs from "dayjs";
    import ProcessArticleFormDialog from "$lib/components/process-article-form-dialog.svelte";
    import Popover from "$lib/components/ui/popover/popover.svelte";
    import PopoverContent from "$lib/components/ui/popover/popover-content.svelte";

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
    <article class="mx-auto prose max-w-4xl py-8 prose-neutral dark:prose-invert">
        {#if article.data.favicon}
            <div class="size-8 pb-16">
                <img src={article.data.favicon} alt="favicon" />
            </div>
        {/if}
        <h1>{article.data.title}</h1>
        {@html article.data.content}
    </article>

    <StickyBottomBar>
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

        {@render menu(article.data)}
    </StickyBottomBar>
{/if}

{#snippet menu(/** @type {import('$lib/api/articles').Article} */ article)}
    <Menu>
        {#snippet trigger(props)}
            <Action button size="sm" variant="outline" {...props}>
                <EllipsisVertical />
                <span>Actions</span>
            </Action>
        {/snippet}
        <MenuContent>
            <MenuGroup>
                <MenuLabel>Article actions</MenuLabel>
                <MenuActionItem anchor href={article.url} target="_blank">
                    <ExternalLink />
                    <span>View external</span>
                </MenuActionItem>
                {#if article.status !== "pending"}
                    <ProcessArticleFormDialog articleId={article.id}>
                        {#snippet trigger(props)}
                            <MenuActionItem button keepOpen {...props}>Process</MenuActionItem>
                        {/snippet}
                    </ProcessArticleFormDialog>
                {/if}
                {#if article.status !== "pending"}
                    <DeleteArticleAlertDialog
                        articleId={article.id}
                        onSuccess={() => goto("/articles")}
                    >
                        {#snippet trigger(props)}
                            <MenuActionItem button keepOpen variant="destructive" {...props}>
                                <Trash />
                                <span>Delete</span>
                            </MenuActionItem>
                        {/snippet}
                    </DeleteArticleAlertDialog>
                {/if}
            </MenuGroup>
        </MenuContent>
    </Menu>
{/snippet}
