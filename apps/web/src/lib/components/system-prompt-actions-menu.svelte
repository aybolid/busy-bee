<script>
    import DeleteSystemPromptDialog from "./delete-system-prompt-dialog.svelte";
    import Trash from "./ui/icons/trash.svelte";
    import MenuActionItem from "./ui/menu/menu-action-item.svelte";
    import MenuContent from "./ui/menu/menu-content.svelte";
    import MenuGroup from "./ui/menu/menu-group.svelte";
    import MenuLabel from "./ui/menu/menu-label.svelte";
    import Menu from "./ui/menu/menu.svelte";

    /** @type {import('svelte').ComponentProps<typeof Menu> & { systemPrompt: import('$lib/api/prompts').SystemPrompt, withoutView?: boolean, onDelete?: () => Promise<void> | void }} */
    const { systemPrompt, withoutView = false, onDelete, ...props } = $props();
</script>

<Menu {...props}>
    <MenuContent>
        <MenuGroup>
            <MenuLabel>Prompt actions</MenuLabel>
            {#if !withoutView}
                <MenuActionItem anchor href="/prompts/system/{systemPrompt.id}">
                    View
                </MenuActionItem>
            {/if}
            <DeleteSystemPromptDialog systemPromptId={systemPrompt.id} onSuccess={onDelete}>
                {#snippet trigger(props)}
                    <MenuActionItem button keepOpen variant="destructive" {...props}>
                        <Trash />
                        <span>Delete</span>
                    </MenuActionItem>
                {/snippet}
            </DeleteSystemPromptDialog>
        </MenuGroup>
    </MenuContent>
</Menu>
