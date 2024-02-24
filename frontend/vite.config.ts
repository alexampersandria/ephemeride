import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import * as path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  build: {
    outDir: '../www',
    emptyOutDir: true,
  },
  resolve: {
    alias: {
      '@': path.join(__dirname, './src'),
    },
  },
})
