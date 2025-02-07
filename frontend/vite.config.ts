import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import * as path from 'path'

// https://vite.dev/config/
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
