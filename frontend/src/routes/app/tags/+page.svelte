<script lang="ts">
import Categories from '$lib/assemblies/Categories.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import { Tags } from 'lucide-svelte'

let dataStore = useDataStore()
</script>

<div class="app-page tags-page">
  <div class="container">
    <div class="app-page-title">
      <Tags />
      Tags
    </div>

    <div class="categories">
      {#if dataStore.categories}
        <Categories categories={dataStore.categories} mode="view" />
      {:else}
        <div class="muted">No tags</div>
      {/if}
    </div>
  </div>
</div>
