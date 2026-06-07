<script>
    import DeleteArticleAlertDialog from "./delete-article-alert-dialog.svelte";
    import ProcessArticleFormDialog from "./process-article-form-dialog.svelte";
    import ExternalLink from "./ui/icons/external-link.svelte";
    import Trash from "./ui/icons/trash.svelte";
    import MenuActionItem from "./ui/menu/menu-action-item.svelte";
    import MenuContent from "./ui/menu/menu-content.svelte";
    import MenuGroup from "./ui/menu/menu-group.svelte";
    import MenuLabel from "./ui/menu/menu-label.svelte";
    import Menu from "./ui/menu/menu.svelte";

    /** @type {import('svelte').ComponentProps<typeof Menu> & { article: import('$lib/api/articles').Article, withoutView?: boolean }} */
    const { article, withoutView = false, ...props } = $props();
</script>

<Menu {...props}>
    <MenuContent>
        <MenuGroup>
            <MenuLabel>Article actions</MenuLabel>
            {#if !withoutView}
                <MenuActionItem anchor href="/articles/{article.id}">View</MenuActionItem>
            {/if}
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
                <DeleteArticleAlertDialog articleId={article.id}>
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
