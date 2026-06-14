<script>
    import DeleteInstructionPromptDialog from "./delete-instruction-prompt-dialog.svelte";
    import EditInstructionPromptFormDialog from "./edit-instruction-prompt-form-dialog.svelte";
    import Trash from "./ui/icons/trash.svelte";
    import MenuActionItem from "./ui/menu/menu-action-item.svelte";
    import MenuContent from "./ui/menu/menu-content.svelte";
    import MenuGroup from "./ui/menu/menu-group.svelte";
    import MenuLabel from "./ui/menu/menu-label.svelte";
    import Menu from "./ui/menu/menu.svelte";

    /** @type {import('svelte').ComponentProps<typeof Menu> & { instructionPrompt: import('$lib/api/prompts').InstructionPrompt, onDelete?: () => Promise<void> | void }} */
    const { instructionPrompt, onDelete, ...props } = $props();
</script>

<Menu {...props}>
    <MenuContent>
        <MenuGroup>
            <MenuLabel>Prompt actions</MenuLabel>
            <EditInstructionPromptFormDialog {instructionPrompt}>
                {#snippet trigger(props)}
                    <MenuActionItem button keepOpen {...props}>Edit</MenuActionItem>
                {/snippet}
            </EditInstructionPromptFormDialog>
            <DeleteInstructionPromptDialog
                instructionPromptId={instructionPrompt.id}
                onSuccess={onDelete}
            >
                {#snippet trigger(props)}
                    <MenuActionItem button keepOpen variant="destructive" {...props}>
                        <Trash />
                        <span>Delete</span>
                    </MenuActionItem>
                {/snippet}
            </DeleteInstructionPromptDialog>
        </MenuGroup>
    </MenuContent>
</Menu>
