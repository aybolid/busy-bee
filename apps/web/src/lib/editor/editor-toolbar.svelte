<script module>
    const HEADINGS = /** @type {const} */ ([{ level: 1 }, { level: 2 }, { level: 3 }]);

    const FORMATTERS = /** @type {const} */ ([
        { type: "bold", toggler: "toggleBold" },
        { type: "italic", toggler: "toggleItalic" },
        { type: "underline", toggler: "toggleUnderline" },
        { type: "strike", toggler: "toggleStrike" },
    ]);
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
    import Separator from "$lib/components/ui/separator.svelte";
    import Bold from "$lib/components/ui/icons/bold.svelte";
    import Italic from "$lib/components/ui/icons/italic.svelte";
    import Strike from "$lib/components/ui/icons/strike.svelte";
    import Underline from "$lib/components/ui/icons/underline.svelte";
    import ArrowLeft from "$lib/components/ui/icons/arrow-left.svelte";
    import ArrowRight from "$lib/components/ui/icons/arrow-right.svelte";

    /** @type {Omit<import('svelte').ComponentProps<typeof StickyBar>, 'children'> & { editor: import('@tiptap/core').Editor }} */
    const { editor, ...props } = $props();

    function getSelectedNodeLabel() {
        const parent = editor.view.state.selection.$head.parent;

        switch (parent.type.name) {
            case "heading":
                return `Heading ${parent.attrs.level}`;
            default:
                return "Paragraph";
        }
    }
</script>

<StickyBar {...props}>
    <Menu>
        {#snippet trigger(props)}
            <Action
                button
                variant={editor.isActive("heading") ? "secondary" : "outline"}
                class="w-30"
                {...props}
            >
                <span>{getSelectedNodeLabel()}</span>
                <ChevronDown />
            </Action>
        {/snippet}
        <MenuContent>
            {#each HEADINGS as heading (heading.level)}
                <MenuActionItem
                    button
                    onclick={() => editor.chain().focus().toggleHeading(heading).run()}
                    variant={editor.isActive("heading", heading) ? "secondary" : undefined}
                    disabled={!editor.can().toggleHeading(heading)}
                    keepOpen
                >
                    {#if heading.level === 1}
                        <Heading1 />
                    {:else if heading.level === 2}
                        <Heading2 />
                    {:else if heading.level === 3}
                        <Heading3 />
                    {/if}
                    <span>Heading {heading.level}</span>
                </MenuActionItem>
            {/each}

            <MenuActionItem
                button
                onclick={() => editor.chain().focus().setParagraph().run()}
                variant={editor.isActive("paragraph") ? "secondary" : undefined}
                disabled={!editor.can().setParagraph()}
                keepOpen
            >
                <Text />
                <span>Paragraph</span>
            </MenuActionItem>
        </MenuContent>
    </Menu>

    <Separator orientation="vertical" />

    <div>
        {#each FORMATTERS as formatter}
            <Action
                button
                size="icon"
                onclick={() => editor.chain().focus()[formatter.toggler]().run()}
                variant={editor.isActive(formatter.type) ? "secondary" : "outline"}
                disabled={!editor.can()[formatter.toggler]()}
            >
                {@render formatterIcon(formatter.type)}
            </Action>
        {/each}
    </div>

    <Separator orientation="vertical" />

    <div>
        <Action
            button
            size="icon"
            variant="secondary"
            onclick={() => editor.chain().focus().undo().run()}
            disabled={!editor.can().undo()}
        >
            <ArrowLeft />
        </Action>
        <Action
            button
            size="icon"
            variant="secondary"
            onclick={() => editor.chain().focus().redo().run()}
            disabled={!editor.can().redo()}
        >
            <ArrowRight />
        </Action>
    </div>
</StickyBar>

{#snippet formatterIcon(/** @type {(typeof FORMATTERS)[number]['type']} */ type)}
    {#if type === "bold"}
        <Bold />
    {:else if type === "italic"}
        <Italic />
    {:else if type === "strike"}
        <Strike />
    {:else if type === "underline"}
        <Underline />
    {/if}
{/snippet}
