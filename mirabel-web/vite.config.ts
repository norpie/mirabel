import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [sveltekit()],
    optimizeDeps: {
        exclude: ['@battlefieldduck/xterm-svelte']
    },
    build: {
        rollupOptions: {
            external: (id) => {
                return id.startsWith('node:')
            }
        }
    }
});
