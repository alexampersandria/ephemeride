import { mdsvex } from 'mdsvex'
import adapter from '@sveltejs/adapter-static'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

const config = {
  kit: {
    adapter: adapter({
      pages: '../www',
      assets: '../www',
      precompress: false,
      strict: true,
      fallback: 'index.html',
    }),
  },
  preprocess: [
    mdsvex({
      extension: '.md',
    }),
    vitePreprocess(),
  ],
  extensions: ['.svelte', '.svx', '.md'],
}

export default config
