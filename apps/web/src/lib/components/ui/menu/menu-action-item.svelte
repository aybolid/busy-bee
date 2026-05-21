<script>
    import Action from "../action.svelte";
    import { cn } from "../utils";

    /** @type {import('../action.svelte').ActionProps} */
    const { children, ...props } = $props();
</script>

<Action
    {...props}
    size={props.size ?? "sm"}
    variant={props.variant ?? "ghost"}
    class={cn("w-full justify-start rounded-md", props.class)}
>
    {@render children?.()}
</Action>
