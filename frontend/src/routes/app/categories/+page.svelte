<script lang="ts">
import Categories from '$lib/assemblies/Categories.svelte'
import { FolderOpen } from 'lucide-svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'

let dataStore = useDataStore()
</script>

<div class="categories-page">
  <div class="container">
    <div class="title">
      <FolderOpen />
      Categories
    </div>

    <div class="categories">
      {#if dataStore.categories}
        <Categories
          categories={dataStore.categories}
          mode="edit"
          categoryAddTag={async tag => {
            return dataStore.createTag(tag)
          }}
          categoryEditTag={async tag => {
            return dataStore.updateTag(tag)
          }}
          categoryRemoveTag={async tagId => {
            return dataStore.deleteTag(tagId)
          }}
          onAddCategory={async category => {
            return dataStore.createCategory(category)
          }}
          onEditCategory={async category => {
            return dataStore.updateCategory(category)
          }}
          onDeleteCategory={async categoryId => {
            return dataStore.deleteCategory(categoryId)
          }} />
      {:else}
        <div class="muted">No categories available</div>
      {/if}
    </div>
  </div>
</div>

<style lang="scss">
.categories-page {
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
