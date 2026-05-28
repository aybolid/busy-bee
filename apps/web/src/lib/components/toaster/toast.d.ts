import type { HTMLAttributes } from "svelte/elements";
import type { ToastData } from "./store";
import type { ToastVariants } from "./toast-variants";

export type ToastProps = { toast: ToastData } & ToastVariants &
    Omit<HTMLAttributes<HTMLDivElement>, "children">;
