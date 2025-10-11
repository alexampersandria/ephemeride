<script lang="ts">
import Chip from '$lib/components/Chip.svelte'
import type { CategoryProps } from '$lib/types/assemblies/category'
import { Pencil, PencilOff, Plus, Tag, X } from 'lucide-svelte'

let {
  name,
  tags = $bindable([]),
  selectedTagIds = $bindable([]),
  mode = 'view',
}: CategoryProps = $props()

const onclick = (tag: { id: string }) => {
  if (mode === 'select') {
    if (selectedTagIds.includes(tag.id)) {
      selectedTagIds = selectedTagIds.filter(id => id !== tag.id)
    } else {
      selectedTagIds = [...selectedTagIds, tag.id]
    }
  } else if (mode === 'edit') {
    // #TODO: edit name or color or remove tag
  }
}

const onedit = () => {
  if (mode === 'select') {
    mode = 'edit'
  } else if (mode === 'edit') {
    mode = 'select'
  }
}

const onclear = () => {
  // remove only tags in category from selectedTagIds
  selectedTagIds = selectedTagIds.filter(id => !tags.find(tag => tag.id === id))
}

const onadd = () => {
  // #TODO: add new tag
}

const _removeTag = (tagId: string) => {
  if (mode === 'edit') {
    tags = tags.filter(tag => tag.id !== tagId)
    selectedTagIds = selectedTagIds.filter(id => id !== tagId)
  }
}
</script>

<div class="category">
  <div class="category-info">
    <div class="category-name">
      {name}
    </div>

    {#if mode === 'select' || mode === 'edit'}
      <div class="category-actions">
        {#if mode === 'edit'}
          <button onclick={onadd} aria-label="Add tag">
            <Chip>
              <Plus />
            </Chip>
          </button>
        {/if}

        {#if mode === 'select' && selectedTagIds.some( id => tags.find(tag => tag.id === id), )}
          <button onclick={onclear} aria-label="Clear selected {name} tags">
            <Chip>
              <X />
            </Chip>
          </button>
        {/if}

        <button onclick={onedit} aria-label="Edit {name} category">
          <Chip>
            {#if mode === 'select'}
              <Pencil />
            {:else if mode === 'edit'}
              <PencilOff />
            {/if}
          </Chip>
        </button>
      </div>
    {/if}
  </div>

  <div class="category-tags">
    {#if tags.length === 0}
      <div class="category-no-tags dimmed">
        <Tag />
        No tags
      </div>
    {:else}
      {#each tags as tag}
        <button
          class="plain"
          onclick={() => onclick(tag)}
          role="option"
          aria-selected={selectedTagIds.includes(tag.id)}
          aria-label={mode === 'select'
            ? selectedTagIds.includes(tag.id)
              ? `Deselect tag: ${tag.name}`
              : `Select tag: ${tag.name}`
            : tag.name}>
          <Chip
            color={tag.color}
            variant={selectedTagIds.includes(tag.id) ? 'solid' : 'subtle'}>
            {tag.name}
          </Chip>
        </button>
      {/each}
    {/if}
  </div>
</div>

<style lang="scss">
.category {
  display: flex;
  flex-direction: column;
  gap: var(--padding-xs);
  width: 100%;

  .category-no-tags {
    font-size: var(--font-size-s);
  }

  .category-info {
    display: flex;
    justify-content: space-between;

    .category-name {
      font-weight: 600;
      font-size: var(--font-size-l);
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .category-actions {
      display: flex;
      gap: var(--padding-xs);
    }
  }

  .category-tags {
    display: flex;
    flex-wrap: wrap;
    gap: var(--padding-xs);
    max-width: 100%;
  }
}
</style>
