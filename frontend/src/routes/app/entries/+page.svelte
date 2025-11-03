<script lang="ts">
import Spinner from '$lib/components/Spinner.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import type { Entry } from '$lib/types/log'
import { ScrollText } from 'lucide-svelte'
import { onMount } from 'svelte'

let dataStore = useDataStore()
let list: Entry[] = $state([])
let loaded = $state(false)

onMount(async () => {
  list = (await dataStore.getAllEntries()) || []
  loaded = true
})
</script>

<div class="entries-page">
  <div class="container">
    <div class="title">
      <ScrollText />
      All entries (WIP)
    </div>

    {#if !loaded}
      <Spinner />
    {:else if list.length === 0}
      <div class="dimmed">No entries found</div>
    {:else}
      <ul class="dimmed">
        {#each list as entry}
          <li>
            <a href={`/app/entry/${entry.date}`}>{entry.date} ({entry.mood})</a>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style lang="scss">
.entries-page {
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
