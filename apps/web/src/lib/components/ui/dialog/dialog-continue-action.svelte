<script>
    import Action from "../action.svelte";
    import { getDialogContext } from "./context";

    /** @type {import('$lib/components/ui/action.svelte').ButtonProps & { closeOnClick?: boolean }} */
    const { children, closeOnClick = false, ...props } = $props();

    const { dialogId } = getDialogContext();
</script>

<Action button {...props} {...closeOnClick ? { command: "close", commandfor: dialogId } : {}}>
    {@render children?.()}
</Action>
