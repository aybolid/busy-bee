<script>
    import { toaster } from "./store";
    import Toast from "./toast.svelte";

    /** @type {{ children: import('svelte').Snippet }} */
    const { children } = $props();

    /** @type {HTMLDivElement} */
    let container;

    $effect(() => {
        const toastCount = $toaster.length;
        if (!container) return;

        if (toastCount > 0) {
            if (container.matches(":popover-open")) {
                container.hidePopover();
            }
            container.showPopover();
        } else {
            if (container.matches(":popover-open")) {
                container.hidePopover();
            }
        }
    });
</script>

{@render children()}

<div
    bind:this={container}
    popover="manual"
    class="fixed inset-auto top-4 left-1/2 m-0 grid w-72 -translate-x-1/2 gap-2 overflow-visible border-0 bg-transparent p-0"
>
    {#each $toaster.slice(0, 3) as toast (toast.id)}
        <Toast {toast} {...toast.props} />
    {/each}
</div>
