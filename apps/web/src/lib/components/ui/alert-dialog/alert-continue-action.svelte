<script>
    import Action from "../action.svelte";
    import { getAlertDialogContext } from "./context";

    /** @type {import('$lib/components/ui/action.svelte').ButtonProps & { closeOnClick?: boolean }} */
    const { children, closeOnClick = false, ...props } = $props();

    const { dialogId } = getAlertDialogContext();
</script>

<Action button {...props} {...closeOnClick ? { command: "close", commandfor: dialogId } : {}}>
    {#if children}
        {@render children()}
    {:else}
        <span>Continue</span>
    {/if}
</Action>
