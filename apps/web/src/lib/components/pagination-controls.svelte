<script>
    import PaginationAction from "./ui/pagination/pagination-action.svelte";
    import PaginationContent from "./ui/pagination/pagination-content.svelte";
    import PaginationEllipsis from "./ui/pagination/pagination-ellipsis.svelte";
    import PaginationItem from "./ui/pagination/pagination-item.svelte";
    import PaginationNext from "./ui/pagination/pagination-next.svelte";
    import PaginationPrevious from "./ui/pagination/pagination-previous.svelte";
    import Pagination from "./ui/pagination/pagination.svelte";

    /**
     * @typedef {Object} PaginationControlsProps
     * @property {number} pageIndex
     * @property {number} totalPages
     * @property {(pageIndex: number) => string} href
     */

    /** @type {Omit<import('$lib/components/ui/pagination/pagination.svelte').PaginationProps, 'children'> & PaginationControlsProps} */
    const { pageIndex, totalPages, href, ...props } = $props();

    /** @type {HTMLAnchorElement} */
    // svelte-ignore non_reactive_update
    let prevAnchor;
    /** @type {HTMLAnchorElement} */
    // svelte-ignore non_reactive_update
    let nextAnchor;

    const isFirstPage = $derived(pageIndex <= 0);
    const isLastPage = $derived(pageIndex >= Math.max(totalPages - 1, 0));

    const pages = $derived.by(() => {
        // If 7 or fewer pages, just show all of them
        if (totalPages <= 7) {
            return Array.from({ length: totalPages }, (_, i) => i);
        }

        // Near the beginning: 1, 2, 3, 4, 5, ..., Last
        if (pageIndex <= 3) {
            return [0, 1, 2, 3, 4, "ellipsis-right", totalPages - 1];
        }

        // Near the end: 1, ..., Last-4, Last-3, Last-2, Last-1, Last
        if (pageIndex >= totalPages - 4) {
            return [
                0,
                "ellipsis-left",
                totalPages - 5,
                totalPages - 4,
                totalPages - 3,
                totalPages - 2,
                totalPages - 1,
            ];
        }

        // In the middle: 1, ..., Current-1, Current, Current+1, ..., Last
        return [
            0,
            "ellipsis-left",
            pageIndex - 1,
            pageIndex,
            pageIndex + 1,
            "ellipsis-right",
            totalPages - 1,
        ];
    });
</script>

<svelte:window
    onkeydown={function (event) {
        if (event.repeat || !event.ctrlKey) return;

        switch (event.code) {
            case "ArrowLeft":
                prevAnchor.click();
                break;
            case "ArrowRight":
                nextAnchor.click();
                break;
        }
    }}
/>

<Pagination {...props}>
    <PaginationContent>
        <PaginationItem>
            <PaginationPrevious
                anchor
                bind:ref={prevAnchor}
                data-sveltekit-noscroll
                class={[isFirstPage && "pointer-events-none opacity-50"]}
                aria-disabled={isFirstPage}
                href={href(!isFirstPage ? pageIndex - 1 : 0)}
            />
        </PaginationItem>

        {#each pages as page (page)}
            {#if typeof page === "string" && page.startsWith("ellipsis")}
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
            {:else}
                <PaginationItem>
                    <PaginationAction
                        anchor
                        data-sveltekit-noscroll
                        href={href(Number(page))}
                        isActive={page === pageIndex}
                    >
                        {Number(page) + 1}
                    </PaginationAction>
                </PaginationItem>
            {/if}
        {/each}

        <PaginationItem>
            <PaginationNext
                anchor
                bind:ref={nextAnchor}
                data-sveltekit-noscroll
                class={[isLastPage && "pointer-events-none opacity-50"]}
                aria-disabled={isLastPage}
                href={href(!isLastPage ? pageIndex + 1 : pageIndex)}
            />
        </PaginationItem>
    </PaginationContent>
</Pagination>
