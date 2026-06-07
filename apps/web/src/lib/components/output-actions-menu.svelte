<script>
    import DeleteOutputDialog from "./delete-output-dialog.svelte";
    import Trash from "./ui/icons/trash.svelte";
    import MenuActionItem from "./ui/menu/menu-action-item.svelte";
    import MenuContent from "./ui/menu/menu-content.svelte";
    import MenuGroup from "./ui/menu/menu-group.svelte";
    import MenuLabel from "./ui/menu/menu-label.svelte";
    import Menu from "./ui/menu/menu.svelte";
    import ViewTokenUsageDialog from "./view-token-usage-dialog.svelte";

    /** @type {import('svelte').ComponentProps<typeof Menu> & { output: import('$lib/api/outputs').Output, withoutView?: boolean }} */
    const { output, withoutView = false, ...props } = $props();
</script>

<Menu {...props}>
    <MenuContent>
        <MenuGroup>
            <MenuLabel>Output actions</MenuLabel>
            {#if !withoutView}
                <MenuActionItem anchor href="/outputs/{output.id}">View</MenuActionItem>
            {/if}
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
            <DeleteOutputDialog outputId={output.id}>
                {#snippet trigger(props)}
                    <MenuActionItem button keepOpen variant="destructive" {...props}>
                        <Trash />
                        <span>Delete</span>
                    </MenuActionItem>
                {/snippet}
            </DeleteOutputDialog>
        </MenuGroup>
    </MenuContent>
</Menu>
