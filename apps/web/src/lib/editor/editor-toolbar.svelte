<script module>
    const HEADINGS = /** @type {const} */ ([{ level: 1 }, { level: 2 }, { level: 3 }]);
</script>

<script>
    import StickyBar from "$lib/components/ui/sticky-bar.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import Menu from "$lib/components/ui/menu/menu.svelte";
    import MenuContent from "$lib/components/ui/menu/menu-content.svelte";
    import ChevronDown from "$lib/components/ui/icons/chevron-down.svelte";
    import MenuActionItem from "$lib/components/ui/menu/menu-action-item.svelte";
    import Text from "$lib/components/ui/icons/text.svelte";
    import Heading1 from "$lib/components/ui/icons/heading-1.svelte";
    import Heading2 from "$lib/components/ui/icons/heading-2.svelte";
    import Heading3 from "$lib/components/ui/icons/heading-3.svelte";
    import { cn } from "$lib/components/ui/utils";

    /** @type {Omit<import('svelte').ComponentProps<typeof StickyBar>, 'children'> & { editor: import('@tiptap/core').Editor }} */
    const { editor, ...props } = $props();

    function selectedNodeIsAnyHeading() {
        return editor.isActive("heading");
    }
</script>

<StickyBar {...props} class={cn("gap-2", props.class)}>
    <Menu>
        {#snippet trigger(props)}
            <Action button variant={selectedNodeIsAnyHeading() ? "secondary" : "ghost"} {...props}>
                <Text />
                <ChevronDown />
            </Action>
        {/snippet}
        <MenuContent>
            {#each HEADINGS as heading}
                <MenuActionItem
                    button
                    onclick={() => editor.chain().focus().toggleHeading(heading).run()}
                    variant={editor.isActive("heading", heading) ? "secondary" : undefined}
                    disabled={!editor.can().toggleHeading(heading)}
                >
                    {#if heading.level === 1}
                        <Heading1 />
                    {:else if heading.level === 2}
                        <Heading2 />
                    {:else if heading.level === 3}
                        <Heading3 />
                    {/if}
                    <span>Toggle <code>H{heading.level}</code></span>
                </MenuActionItem>
            {/each}

            <MenuActionItem
                button
                onclick={() => editor.chain().focus().setParagraph().run()}
                variant={editor.isActive("paragraph") ? "secondary" : undefined}
                disabled={!editor.can().setParagraph()}
            >
                <Text />
                <span>Paragraph</span>
            </MenuActionItem>
        </MenuContent>
    </Menu>
</StickyBar>
