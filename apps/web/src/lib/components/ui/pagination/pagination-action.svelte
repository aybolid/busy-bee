<script module>
    /** @typedef {import('../action.svelte').ActionProps & { isActive?: boolean }} PaginationActionProps */
</script>

<script>
    import Action from "../action.svelte";

    /** @type {PaginationActionProps} */
    const { children, isActive, ...props } = $props();
</script>

<Action
    aria-current={isActive ? "page" : undefined}
    {...props}
    size={props.size ?? "icon"}
    variant={props.variant ?? (isActive ? "outline" : "ghost")}
>
    {@render children?.()}
</Action>
