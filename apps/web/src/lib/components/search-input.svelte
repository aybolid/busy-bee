<script>
    import Input from "./ui/input.svelte";

    /** @type {Omit<import('svelte').ComponentProps<typeof Input>, 'oninput'> & { onDebouncedInput?: (v: string) => void }} */
    const { onDebouncedInput, ...props } = $props();

    /** @type {ReturnType<typeof setTimeout>} */
    let timer;

    /** @type {import('svelte/elements').FormEventHandler<HTMLInputElement>} */
    function handleInput(e) {
        if (!onDebouncedInput) return;

        const value = e.currentTarget.value.trim();

        clearTimeout(timer);

        if (!value || value.length < 2) {
            onDebouncedInput("");
        } else {
            timer = setTimeout(() => {
                onDebouncedInput(value);
            }, 300);
        }
    }

    $effect(() => () => clearTimeout(timer));
</script>

<Input {...props} oninput={handleInput} />
