<script module>
    const NUMBER_FORMAT = new Intl.NumberFormat("en-US");
</script>

<script>
    import { createQuery } from "@tanstack/svelte-query";
    import { getRssFeedsQueryOptions } from "$lib/query/rss-feeds";
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
    import Card from "$lib/components/ui/card/card.svelte";
    import CardTitle from "$lib/components/ui/card/card-title.svelte";
    import CardHeader from "$lib/components/ui/card/card-header.svelte";
    import Badge from "$lib/components/ui/badge.svelte";
    import { cn } from "$lib/components/ui/utils";
    import CardContent from "$lib/components/ui/card/card-content.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import ExternalLink from "$lib/components/ui/icons/external-link.svelte";
    import CardFooter from "$lib/components/ui/card/card-footer.svelte";
    import Empty from "$lib/components/ui/empty/empty.svelte";
    import EmptyHeader from "$lib/components/ui/empty/empty-header.svelte";
    import EmptyTitle from "$lib/components/ui/empty/empty-title.svelte";
    import EmptyDescription from "$lib/components/ui/empty/empty-description.svelte";
    import Plus from "$lib/components/ui/icons/plus.svelte";
    import EmptyContent from "$lib/components/ui/empty/empty-content.svelte";
    import CreateFeedFormDialog from "./create-feed-form-dialog.svelte";
    import Trash from "$lib/components/ui/icons/trash.svelte";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    const feeds = createQuery(() => getRssFeedsQueryOptions(props.data.ky));
</script>

<div class="flex items-baseline justify-between gap-8 pb-8">
    <h1 class="text-4xl font-bold">RSS feeds</h1>
    {#if (feeds.data?.length ?? 0) !== 0}
        <CreateFeedFormDialog>
            {#snippet trigger(props)}
                <Action button {...props}>
                    <Plus />
                    <span>RSS feed</span>
                </Action>
            {/snippet}
        </CreateFeedFormDialog>
    {/if}
</div>

{#if feeds.isPending}
    <Pending />
{:else if feeds.isError}
    <ErrorAlert error={feeds.error} />
{:else if feeds.isSuccess}
    {#if feeds.data.length === 0}
        <Empty>
            <EmptyHeader>
                <EmptyTitle>No RSS feeds</EmptyTitle>
                <EmptyDescription>There are no feeds to display.</EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <CreateFeedFormDialog>
                    {#snippet trigger(props)}
                        <Action button {...props}>
                            <Plus />
                            <span>RSS feed</span>
                        </Action>
                    {/snippet}
                </CreateFeedFormDialog>
            </EmptyContent>
        </Empty>
    {:else}
        <div class="grid grid-cols-3 gap-4">
            {#each feeds.data as feed (feed.id)}
                {@const url = new URL(feed.url)}
                <Card class={cn(feed.status === "error" && "ring-2 ring-destructive/30")} size="sm">
                    <CardHeader>
                        <div class="flex items-baseline gap-2">
                            <Badge
                                class="capitalize"
                                variant={feed.status === "error" ? "destructive" : "default"}
                            >
                                {feed.status}
                            </Badge>
                            <CardTitle>{url.hostname}</CardTitle>
                        </div>
                    </CardHeader>
                    <CardContent class="flex h-full flex-col gap-4">
                        <ul class="space-y-1">
                            <li class="flex items-baseline justify-between gap-4">
                                <span class="text-muted-foreground">Max concurrency</span>
                                <Badge variant="secondary">
                                    {NUMBER_FORMAT.format(feed.max_concurrent_requests)}
                                </Badge>
                            </li>
                            <li class="flex items-baseline justify-between gap-4">
                                <span class="text-muted-foreground">Fetch interval</span>
                                <Badge variant="secondary">
                                    {NUMBER_FORMAT.format(feed.fetch_interval_seconds)}s
                                </Badge>
                            </li>
                        </ul>
                        {#if feed.status === "error"}
                            <ErrorAlert description={feed.error_reason} />
                        {/if}
                    </CardContent>
                    <CardFooter class="justify-between gap-4">
                        <Action anchor href={feed.url} variant="link" size="xs" target="_blank">
                            <ExternalLink />
                            <span>
                                {feed.url}
                            </span>
                        </Action>
                        <Action button size="icon-xs" variant="destructive">
                            <Trash />
                        </Action>
                    </CardFooter>
                </Card>
            {/each}
        </div>
    {/if}
{/if}
