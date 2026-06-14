<script>
    import Search from "./ui/icons/search.svelte";
    import InputGroupAddon from "./ui/input-group/input-group-addon.svelte";
    import InputGroup from "./ui/input-group/input-group.svelte";
    import InputGroupInput from "./ui/input-group/input-group-input.svelte";

    /** @type {import('svelte').ComponentProps<typeof InputGroup> & { onDebouncedInput?: (v: string) => void, placeholder?: string, value?: string }} */
    const { onDebouncedInput, placeholder, value, ...props } = $props();

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

<InputGroup {...props}>
    <InputGroupAddon align="inline-start">
        <Search />
    </InputGroupAddon>
    <InputGroupInput oninput={handleInput} {placeholder} {value} />
</InputGroup>
