<script lang="ts">
import Entry from '$lib/assemblies/Entry.svelte'
import Spinner from '$lib/components/Spinner.svelte'
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

// force reload to reset Entry component when date param changes
let forceReload = $state(false)
$effect(() => {
  if (data.date) {
    forceReload = true
    setTimeout(() => {
      forceReload = false
    }, 0)
  }
})
</script>

<div class="entry-page">
  {#if !forceReload}
    {#if entry && dataStore.categories}
      <Entry
        mode="view"
        id={entry.id}
        date={entry.date}
        entry={entry.entry || ''}
        mood={entry.mood}
        selectedTagIds={entry.selected_tags}
        categories={dataStore.categories}
        onUpdate={entry => {
          dataStore.updateEntry(entry)
        }}
        onDelete={id => {
          dataStore.deleteEntry(id)
        }}
        onAddTag={tag => {
          dataStore.createTag(tag)
        }}
        onEditTag={tag => {
          dataStore.updateTag({
            id: tag.id,
            name: tag.name,
            color: tag.color,
          })
        }}
        onRemoveTag={tagId => {
          dataStore.deleteTag(tagId)
        }}
        onAddCategory={category => {
          dataStore.createCategory(category)
        }}
        onEditCategory={category => {
          dataStore.updateCategory({
            id: category.id,
            name: category.name,
          })
        }}
        onDeleteCategory={categoryId => {
          dataStore.deleteCategory(categoryId)
        }} />
    {:else if dataStore.categories}
      <Entry
        date={data.date}
        mode="create"
        categories={dataStore.categories}
        onCreate={entry => {
          dataStore.createEntry(entry)
        }}
        onAddTag={tag => {
          dataStore.createTag(tag)
        }}
        onEditTag={tag => {
          dataStore.updateTag({
            id: tag.id,
            name: tag.name,
            color: tag.color,
          })
        }}
        onRemoveTag={tagId => {
          dataStore.deleteTag(tagId)
        }}
        onAddCategory={category => {
          dataStore.createCategory(category)
        }}
        onEditCategory={category => {
          dataStore.updateCategory({
            id: category.id,
            name: category.name,
          })
        }}
        onDeleteCategory={categoryId => {
          dataStore.deleteCategory(categoryId)
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
