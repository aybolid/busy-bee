import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig, searchForWorkspaceRoot } from "vite";

export default defineConfig({
    plugins: [tailwindcss(), sveltekit()],
    server: {
        proxy: {
            "/api": "http://localhost:3000",
        },
        fs: {
            allow: [searchForWorkspaceRoot(process.cwd()), "../.."],
        },
    },
});
