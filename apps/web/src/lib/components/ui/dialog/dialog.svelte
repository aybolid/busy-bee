<script module>
    /**
     * @typedef {Object} TriggerContext
     * @property {'show-modal'} command
     * @property {string} commandfor
     */

    /**
     * A snippet that renders the trigger element (e.g., a button) to open the alert dialog.
     * @typedef {import('svelte').Snippet<[TriggerContext]>} TriggerSnippet
     */

    /**
     * @typedef {Object} DialogSnippets
     * @property {TriggerSnippet} [trigger] Optional snippet to render the dialog's trigger mechanism.
     */

    /**
     * @typedef {import('svelte/elements').HTMLAttributes<HTMLDivElement> & DialogSnippets & { ref?: HTMLDialogElement }} DialogProps
     */
</script>

<script>
    import { cn } from "../utils";
    import { setDialogContext } from "./context";

    const id = $props.id();
    const context = setDialogContext({
        dialogId: id,
        descriptionId: `${id}-description`,
        labelId: `${id}-title`,
    });

    /** @type {DialogProps} */
    let { ref = $bindable(), trigger, children, ...props } = $props();
</script>

<div {...props} class={cn("group contents", props.class)}>
    {@render trigger?.({ command: "show-modal", commandfor: context.dialogId })}

    <dialog
        bind:this={ref}
        id={context.dialogId}
        aria-labelledby={context.labelId}
        aria-describedby={context.descriptionId}
        class="backdrop:bg-black/50 backdrop:backdrop-blur-xs"
    >
        {@render children?.()}
    </dialog>
</div>
