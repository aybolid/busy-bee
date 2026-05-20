<script>
    const id = $props.id();

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
     * @typedef {Object} CloserContext
     * @property {'close'} command The action command to close the dialog.
     * @property {string} commandfor The unique ID of the target dialog element.
     */

    /**
     * @typedef {Object} ContentContext
     * @property {CloserContext} closer Object containing properties required to build a close button.
     */

    /**
     * A snippet that renders the internal content of the `<dialog>` element.
     * @typedef {import('svelte').Snippet<[ContentContext]>} ContentSnippet
     */

    /**
     * @typedef {Object} AlertDialogSnippets
     * @property {TriggerSnippet} [trigger] Optional snippet to render the dialog's trigger mechanism.
     * @property {ContentSnippet} [content] Optional snippet to render the inner dialog content and close controls.
     */

    /**
     * @type {Omit<import('svelte/elements').HTMLAttributes<HTMLDivElement>, 'children'> & AlertDialogSnippets}
     */
    const { trigger, content, ...props } = $props();
</script>

<div {...props} class={["group contents", props.class]}>
    <span>
        {@render trigger?.({ command: "show-modal", commandfor: id })}
    </span>

    <dialog {id} class="backdrop:bg-black/50">
        {@render content?.({ closer: { command: "close", commandfor: id } })}
    </dialog>
</div>
