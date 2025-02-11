<script lang="ts">
import { browser } from '$app/environment'
import '$lib/assets/main.scss'
import Preloader from '$lib/components/Preloader.svelte'
import { i18n } from '$lib/i18n'
import { theme } from '$lib/store/ui.svelte'
import { ParaglideJS } from '@inlang/paraglide-sveltekit'

let { children } = $props()

const showPreloader = !browser
</script>

<ParaglideJS {i18n}>
  <div id="root" class="theme-{theme()}">
    {@render children()}
  </div>

  {#if showPreloader}
    <Preloader />
  {/if}
</ParaglideJS>
