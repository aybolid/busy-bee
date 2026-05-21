<script>
    import Action from "../action.svelte";
    import { cn } from "../utils";
    import { getMenuContext } from "./context";

    /** @type {import('../action.svelte').ActionProps & { keepOpen?: boolean }} */
    const { children, keepOpen = false, ...props } = $props();
    const { menuId } = getMenuContext();

    const closeCommandProps = $derived(
        props.button && !keepOpen
            ? {
                  command: "hide-popover",
                  commandfor: menuId,
              }
            : {},
    );
</script>

<Action
    {...props}
    {...closeCommandProps}
    size={props.size ?? "sm"}
    variant={props.variant ?? "ghost"}
    class={cn("w-full justify-start rounded-md", props.class)}
>
    {@render children?.()}
</Action>
