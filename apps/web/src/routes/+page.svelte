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
    import Trash from "$lib/components/ui/icons/trash.svelte";
    import RssFeedStatus from "$lib/components/rss-feed-status.svelte";
    import Accordion from "$lib/components/ui/accordion/accordion.svelte";
    import AccordionItem from "$lib/components/ui/accordion/accordion-item.svelte";
    import AccordionHeader from "$lib/components/ui/accordion/accordion-header.svelte";
    import AccordionTrigger from "$lib/components/ui/accordion/accordion-trigger.svelte";
    import AccordionContent from "$lib/components/ui/accordion/accordion-content.svelte";
    import CreateFeedFormDialog from "$lib/components/create-feed-form-dialog.svelte";
    import DeleteRssFeedDialog from "$lib/components/delete-rss-feed-dialog.svelte";
    import ItemGroup from "$lib/components/ui/item/item-group.svelte";
    import Item from "$lib/components/ui/item/item.svelte";
    import ItemTitle from "$lib/components/ui/item/item-title.svelte";
    import ItemContent from "$lib/components/ui/item/item-content.svelte";
    import ItemDescription from "$lib/components/ui/item/item-description.svelte";
    import ItemActions from "$lib/components/ui/item/item-actions.svelte";
    import AccordionChevron from "$lib/components/ui/accordion/accordion-chevron.svelte";
    import awesomeRssFeeds from "$lib/rss-feeds.json";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    const feeds = createQuery(() => getRssFeedsQueryOptions(props.data.ky));
    const existingFeedUrls = $derived(new Set(feeds.data?.map((feed) => feed.url) ?? []));
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
        <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
            {#each feeds.data as feed (feed.id)}
                {@const url = feed.parsedUrl()}
                <Card class={cn(feed.status === "error" && "ring-2 ring-destructive/30")}>
                    <CardHeader>
                        <div class="flex items-baseline gap-2">
                            <RssFeedStatus status={feed.status} />
                            <CardTitle>{url.hostname}</CardTitle>
                        </div>
                    </CardHeader>
                    <CardContent class="flex h-full flex-col gap-4">
                        <ul class="space-y-1">
                            <li class="flex items-baseline justify-between gap-4">
                                <span class="text-muted-foreground">Max concurrency</span>
                                <Badge variant="secondary">
                                    {feed.formattedMaxConcurrentRequests()}
                                </Badge>
                            </li>
                            <li class="flex items-baseline justify-between gap-4">
                                <span class="text-muted-foreground">Fetch interval</span>
                                <Badge variant="secondary">
                                    {feed.formattedFetchIntervalSeconds()}
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
                        <DeleteRssFeedDialog feedId={feed.id}>
                            {#snippet trigger(props)}
                                <Action button size="icon-xs" variant="destructive" {...props}>
                                    <Trash />
                                </Action>
                            {/snippet}
                        </DeleteRssFeedDialog>
                    </CardFooter>
                </Card>
            {/each}
        </div>
    {/if}
{/if}

<Accordion class="pt-8">
    <AccordionItem class="rounded-md border border-dashed">
        <AccordionHeader>
            <AccordionTrigger class="px-4">
                <span>Awesome RSS feeds</span>
                <AccordionChevron />
            </AccordionTrigger>
        </AccordionHeader>
        <AccordionContent class="p-4">
            <ItemGroup class="grid gap-2 sm:grid-cols-2 lg:grid-cols-3">
                {#each awesomeRssFeeds as feed}
                    {@const url = new URL(feed.url)}
                    {@const isExistingFeed = existingFeedUrls.has(feed.url)}

                    <Item variant={isExistingFeed ? "default" : "outline"}>
                        <ItemContent>
                            <ItemTitle>{feed.title}</ItemTitle>
                            <ItemDescription>
                                <a href={feed.url} target="_blank">
                                    {url.hostname}
                                </a>
                            </ItemDescription>
                        </ItemContent>
                        <ItemActions>
                            <CreateFeedFormDialog defaultUrl={feed.url}>
                                {#snippet trigger(props)}
                                    <Action
                                        disabled={isExistingFeed}
                                        button
                                        variant={isExistingFeed ? "default" : "outline"}
                                        size="sm"
                                        class="w-full"
                                        {...props}
                                    >
                                        {#if isExistingFeed}
                                            <span>Already added</span>
                                        {:else}
                                            <span>Add this feed</span>
                                        {/if}
                                    </Action>
                                {/snippet}
                            </CreateFeedFormDialog>
                        </ItemActions>
                    </Item>
                {/each}
            </ItemGroup>
        </AccordionContent>
    </AccordionItem>
</Accordion>
