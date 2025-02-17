<script lang="ts">
import 'modern-normalize/modern-normalize.css'

import '$lib/assets/main.scss'

import Preloader from '$lib/components/Preloader.svelte'
import { initializeStores } from '$lib/store/index.svelte'
import { useUiStore } from '$lib/store/uiStore.svelte'

initializeStores()

let uiStore = useUiStore()

let { children } = $props()
</script>

<div id="root" class="theme-{uiStore.appliedTheme}">
  {@render children()}

  {#if uiStore.loading}
    <Preloader />
  {/if}
</div>
