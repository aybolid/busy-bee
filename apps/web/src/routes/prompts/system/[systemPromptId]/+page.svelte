<script>
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import Badge from "$lib/components/ui/badge.svelte";
    import StickyBar from "$lib/components/ui/sticky-bar.svelte";
    import EllipsisVertical from "$lib/components/ui/icons/ellipsis-vertical.svelte";
    import { createQuery } from "@tanstack/svelte-query";
    import { goto } from "$app/navigation";
    import { getSystemPromptQueryOptions } from "$lib/query/prompts";
    import SvelteMarkdown from "@humanspeak/svelte-markdown";
    import SystemPromptActionsMenu from "$lib/components/system-prompt-actions-menu.svelte";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    const systemPromptId = /** @type {import('$lib/api/prompts').SystemPromptId} */ (
        $derived(props.params.systemPromptId)
    );

    const prompt = createQuery(() =>
        getSystemPromptQueryOptions(props.data.ky, { params: { id: systemPromptId } }),
    );
</script>

{#if prompt.isPending}
    <Pending />
{:else if prompt.isError}
    <ErrorAlert error={prompt.error} />
{:else if prompt.isSuccess}
    <article class="mx-auto prose max-w-4xl py-8 prose-app">
        <h1>{prompt.data.title}</h1>
        <hr />
        <SvelteMarkdown source={prompt.data.text} />
    </article>

    <StickyBar>
        <div class="flex flex-wrap gap-2">
            <Badge>Version {prompt.data.version}</Badge>
        </div>

        <SystemPromptActionsMenu
            systemPrompt={prompt.data}
            withoutView
            onDelete={() => goto("/prompts")}
        >
            {#snippet trigger(props)}
                <Action button size="sm" variant="outline" {...props}>
                    <EllipsisVertical />
                    <span>Actions</span>
                </Action>
            {/snippet}
        </SystemPromptActionsMenu>
    </StickyBar>
{/if}
