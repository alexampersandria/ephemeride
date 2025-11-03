<script lang="ts">
import Entry from '$lib/assemblies/Entry.svelte'
import Spinner from '$lib/components/Spinner.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import type { PageProps } from './$types'

let { data }: PageProps = $props()

let dataStore = useDataStore()

let entry = $derived.by(() => {
  if (dataStore.entries) {
    return dataStore.entries[data.date]
  } else {
    return null
  }
})

// force reload to reset Entry component when date param changes
let forceReload = $state(false)
$effect(() => {
  if (data.date) {
    dataStore.getEntry(data.date)
    forceReload = true
    setTimeout(() => {
      forceReload = false
    }, 0)
  }
})
</script>

<div class="entry-page">
  {#if !forceReload}
    {#if dataStore.categories}
      <Entry
        mode={entry ? 'view' : 'create'}
        id={entry ? entry.id : undefined}
        date={data.date}
        entry={entry ? entry.entry : undefined}
        mood={entry ? entry.mood : undefined}
        selectedTagIds={entry ? entry.selected_tags : undefined}
        categories={dataStore.categories}
        onCreate={async entry => {
          return dataStore.createEntry({
            date: data.date,
            entry: entry.entry,
            mood: entry.mood,
            selected_tags: entry.selected_tags,
          })
        }}
        onUpdate={async entry => {
          return dataStore.updateEntry({
            id: entry.id,
            date: data.date,
            entry: entry.entry,
            mood: entry.mood,
            selected_tags: entry.selected_tags,
          })
        }}
        onDelete={async id => {
          return dataStore.deleteEntry(id)
        }}
        onAddTag={async tag => {
          return dataStore.createTag({
            name: tag.name,
            color: tag.color,
            category_id: tag.category_id,
          })
        }}
        onEditTag={async tag => {
          return dataStore.updateTag({
            id: tag.id,
            name: tag.name,
            color: tag.color,
          })
        }}
        onRemoveTag={async tagId => {
          return dataStore.deleteTag(tagId)
        }}
        onAddCategory={async category => {
          return dataStore.createCategory({
            name: category.name,
          })
        }}
        onEditCategory={async category => {
          return dataStore.updateCategory({
            id: category.id,
            name: category.name,
          })
        }}
        onDeleteCategory={async categoryId => {
          return dataStore.deleteCategory(categoryId)
        }} />
    {:else}
      <Spinner />
    {/if}
  {/if}
</div>

<style lang="scss">
.entry-page {
  display: flex;
  justify-content: center;
  padding: var(--padding-l);
}
</style>
