<script lang="ts">
import Entry from '$lib/assemblies/Entry.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import type { Entry as EntryType } from '$lib/types/log'
import type { PageProps } from './$types'

let { data }: PageProps = $props()

let dataStore = useDataStore()

let entry: EntryType | null = $state(null)

$effect(() => {
  if (dataStore) {
    dataStore.getEntry(data.date).then(e => {
      entry = e
    })
  }
})
</script>

<div class="entry-page">
  {#if entry}
    <Entry
      mode="view"
      date={entry.date}
      entry={entry.entry || ''}
      mood={entry.mood}
      selectedTagIds={entry.selectedTagIds} />
  {:else}
    <Entry date={data.date} mode="create" />
  {/if}
</div>

<style lang="scss">
.entry-page {
  display: flex;
  justify-content: center;
  padding: var(--padding-l);
}
</style>
