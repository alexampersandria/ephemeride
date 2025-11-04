<script lang="ts">
import Categories from '$lib/assemblies/Categories.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import { Tags } from 'lucide-svelte'

let dataStore = useDataStore()
</script>

<div class="tags-page">
  <div class="container">
    <div class="title">
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

<style lang="scss">
.tags-page {
  padding: var(--padding-l);

  .title {
    font-size: var(--font-size-l);
    display: flex;
    align-items: center;
    gap: var(--padding-s);
    margin-bottom: var(--padding-m);
  }
}
</style>
