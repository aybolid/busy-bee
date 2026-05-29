<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & { errors?: Array<{ message?: string } | undefined> }} */
    const { children, errors, ...props } = $props();

    const uniqueErrors = $derived.by(() => {
        if (!errors?.length) return [];
        return [...new Map(errors.map((error) => [error?.message, error])).values()];
    });
</script>

{#if uniqueErrors.length !== 0 || children}
    <div
        {...props}
        role="alert"
        data-slot="field-error"
        class={cn("text-start text-sm font-normal text-destructive", props.class)}
    >
        {#if children}
            {@render children()}
        {:else if uniqueErrors.length === 1}
            {uniqueErrors[0]?.message}
        {:else}
            <ul class="ml-4 flex list-disc flex-col gap-1">
                {#each uniqueErrors as error}
                    {#if error?.message}
                        <li>{error.message}</li>
                    {/if}
                {/each}
            </ul>
        {/if}
    </div>
{/if}
