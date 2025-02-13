<script lang="ts">
import '$lib/assets/main.scss'

import Preloader from '$lib/components/Preloader.svelte'
import { useUiStore } from '$lib/store/uiStore.svelte'

let uiStore = useUiStore()

let { children } = $props()
</script>

<div id="root" class="theme-{uiStore.appliedTheme}">
  {@render children()}
</div>

{#if uiStore.loading}
  <Preloader />
{/if}
