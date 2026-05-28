<script>
    import { slide } from "svelte/transition";
    import { cn } from "../ui/utils";
    import { toaster } from "./store";
    import ToastMessage from "./toast-message.svelte";
    import Action from "../ui/action.svelte";
    import X from "../ui/icons/x.svelte";
    import ToastDescription from "./toast-description.svelte";
    import { variants } from "./toast-variants";

    /** @type {import('./toast').ToastProps} */
    const { toast, ...props } = $props();

    $effect(() => {
        const timeout = setTimeout(() => {
            toaster.remove(toast.id);
        }, toast.duration);

        return () => {
            clearTimeout(timeout);
        };
    });
</script>

<div
    transition:slide={{ duration: 100 }}
    {...props}
    class={cn(variants({ variant: props.variant, class: props.class }))}
>
    <ToastMessage>{toast.message}</ToastMessage>
    {#if toast.description}
        <ToastDescription>{toast.description}</ToastDescription>
    {/if}

    <Action
        button
        size="icon-xs"
        variant="secondary"
        class="absolute top-1 right-1"
        onclick={() => toaster.remove(toast.id)}
    >
        <X />
    </Action>
</div>
