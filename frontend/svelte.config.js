import { mdsvex } from 'mdsvex'
import rehypeSlug from 'rehype-slug'
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
      rehypePlugins: [rehypeSlug],
    }),
    vitePreprocess(),
  ],
  extensions: ['.svelte', '.svx', '.md'],
}

export default config
