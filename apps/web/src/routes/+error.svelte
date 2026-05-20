<script>
    import { page } from "$app/state";
    import AlertDescription from "$lib/components/ui/alert/alert-description.svelte";
    import AlertTitle from "$lib/components/ui/alert/alert-title.svelte";
    import Alert from "$lib/components/ui/alert/alert.svelte";
</script>

<Alert variant="destructive">
    <AlertTitle>{page.status} Error</AlertTitle>
    <AlertDescription>
        {page.error?.message}
    </AlertDescription>
</Alert>
