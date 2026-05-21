<script module>
    /** @typedef {import('../action.svelte').ActionProps & { isActive?: boolean }} PaginationActionProps */
</script>

<script>
    import Action from "../action.svelte";

    /** @type {PaginationActionProps} */
    let { children, isActive, ref = $bindable(), ...props } = $props();
</script>

<Action
    bind:ref
    aria-current={isActive ? "page" : undefined}
    {...props}
    size={props.size ?? "icon"}
    variant={props.variant ?? (isActive ? "outline" : "ghost")}
>
    {@render children?.()}
</Action>
