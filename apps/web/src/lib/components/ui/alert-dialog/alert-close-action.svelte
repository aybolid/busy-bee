<script>
    import Action from "../action.svelte";
    import { getAlertDialogContext } from "./context";

    /** @type {import('$lib/components/ui/action.svelte').ButtonProps} */
    const { children, ...props } = $props();

    const { dialogId } = getAlertDialogContext();
</script>

<Action
    button
    {...props}
    variant={props.variant ?? "outline"}
    command="close"
    commandfor={dialogId}
>
    {#if children}
        {@render children()}
    {:else}
        <span>Cancel</span>
    {/if}
</Action>
