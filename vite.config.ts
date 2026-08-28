import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
    plugins: [react()],

    server: {
        port: 1420,
        strictPort: true,

        watch: {
            ignored: [
                "**/.vs/**",
                "**/node_modules/**",
                "**/src-tauri/target/**",
            ],
        },
    },
});