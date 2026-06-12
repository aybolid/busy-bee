<script>
    import { page } from "$app/state";
    import ErrorAlert from "$lib/components/error-alert.svelte";
</script>

<ErrorAlert title={page.status.toString()} description={page.error?.message} />
