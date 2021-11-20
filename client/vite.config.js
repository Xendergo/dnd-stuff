import { defineConfig } from "vite"
import { svelte } from "@sveltejs/vite-plugin-svelte"
import { resolve, dirname } from "path"
import { fileURLToPath } from "url"

// https://stackoverflow.com/questions/46745014/alternative-for-dirname-in-node-when-using-the-experimental-modules-flag
const __dirname = dirname(fileURLToPath(import.meta.url))

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [svelte()],
    base: "",
    build: {
        rollupOptions: {
            input: {
                index: resolve(__dirname, "index.html"),
                "campaign-creator": resolve(
                    __dirname,
                    "campaign-creator/index.html"
                ),
                "character-creator": resolve(
                    __dirname,
                    "character-creator/index.html"
                ),
            },
        },
    },
})
