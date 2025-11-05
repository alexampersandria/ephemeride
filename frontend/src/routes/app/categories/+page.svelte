<script lang="ts">
import Categories from '$lib/assemblies/Categories.svelte'
import { FolderOpen } from 'lucide-svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'

let dataStore = useDataStore()
</script>

<div class="app-page categories-page">
  <div class="container">
    <div class="app-page-title">
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
        <div class="muted">No categories</div>
      {/if}
    </div>
  </div>
</div>
