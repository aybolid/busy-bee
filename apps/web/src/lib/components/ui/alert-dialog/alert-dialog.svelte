<script>
    import { setAlertDialogContext } from "./context";

    const id = $props.id();
    const context = setAlertDialogContext({
        dialogId: id,
        descriptionId: `${id}-description`,
        labelId: `${id}-title`,
    });

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
     * @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & AlertDialogSnippets}
     */
    const { trigger, children, ...props } = $props();
</script>

<div {...props} class={["group contents", props.class]}>
    {@render trigger?.({ command: "show-modal", commandfor: context.dialogId })}

    <dialog
        id={context.dialogId}
        role="alertdialog"
        aria-labelledby={context.labelId}
        aria-describedby={context.descriptionId}
        class="backdrop:bg-black/50"
    >
        {@render children?.()}
    </dialog>
</div>
