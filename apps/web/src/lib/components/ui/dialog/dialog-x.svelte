<script>
    import Action from "../action.svelte";
    import X from "../icons/x.svelte";
    import { cn } from "../utils";
    import { getDialogContext } from "./context";

    /** @type {Omit<import('$lib/components/ui/action.svelte').ButtonProps, 'children'>} */
    const props = $props();

    const { dialogId } = getDialogContext();
</script>

<Action
    button
    {...props}
    variant={props.variant ?? "ghost"}
    size={props.size ?? "icon-sm"}
    class={cn("absolute top-2 right-2", props.class)}
    command="close"
    commandfor={dialogId}
>
    <X />
</Action>
