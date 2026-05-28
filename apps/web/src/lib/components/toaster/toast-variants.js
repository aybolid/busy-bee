import { cva } from "class-variance-authority";

export const variants = cva("relative cursor-default rounded-md border bg-card p-2 shadow-md", {
    variants: {
        variant: {
            default: "",
            destructive: "text-destructive ring ring-2 ring-destructive/30",
        },
    },
    defaultVariants: {
        variant: "default",
    },
});

/** @typedef {import('class-variance-authority').VariantProps<typeof variants>} ToastVariants */
