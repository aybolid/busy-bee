<script>
    import Action from "../action.svelte";
    import { getDialogContext } from "./context";

    /** @type {import('$lib/components/ui/action.svelte').ButtonProps} */
    const { children, ...props } = $props();

    const { dialogId } = getDialogContext();
</script>

<Action
    button
    {...props}
    variant={props.variant ?? "outline"}
    command="close"
    commandfor={dialogId}
>
    {@render children?.()}
</Action>
