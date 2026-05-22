<script module>
    /**
     * @typedef {Object} TriggerContext
     * @property {'show-modal'} command The action command to open the dialog.
     * @property {string} commandfor The unique ID of the target dialog element.
     */

    /**
     * A snippet that renders the trigger element (e.g., a button) to open the alert dialog.
     * @typedef {import('svelte').Snippet<[TriggerContext]>} TriggerSnippet
     */

    /**
     * @typedef {Object} AlertDialogSnippets
     * @property {TriggerSnippet} [trigger] Optional snippet to render the dialog's trigger mechanism.
     */

    /**
     * @typedef {import('svelte/elements').HTMLAttributes<HTMLDivElement> & AlertDialogSnippets & { ref?: HTMLDialogElement }} AlertDialogProps
     */
</script>

<script>
    import { cn } from "../utils";
    import { setAlertDialogContext } from "./context";

    const id = $props.id();
    const context = setAlertDialogContext({
        dialogId: id,
        descriptionId: `${id}-description`,
        labelId: `${id}-title`,
    });

    /** @type {AlertDialogProps} */
    let { trigger, children, ref = $bindable(), ...props } = $props();
</script>

<div {...props} class={cn("group contents", props.class)}>
    {@render trigger?.({ command: "show-modal", commandfor: context.dialogId })}

    <dialog
        bind:this={ref}
        id={context.dialogId}
        role="alertdialog"
        aria-labelledby={context.labelId}
        aria-describedby={context.descriptionId}
        class="backdrop:bg-black/50 backdrop:backdrop-blur-xs"
    >
        {@render children?.()}
    </dialog>
</div>
