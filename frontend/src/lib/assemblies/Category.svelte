<script lang="ts">
import Chip from '$lib/components/Chip.svelte'
import type { CategoryProps } from '$lib/types/assemblies/category'
import { Pencil, Tag, X } from 'lucide-svelte'

let {
  name,
  tags = [],
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
  }
}

const onedit = () => {
  // #TODO
}

const onclear = () => {
  // remove only tags in category from selectedTagIds
  selectedTagIds = selectedTagIds.filter(id => !tags.find(tag => tag.id === id))
}
</script>

<div class="category">
  <div class="category-info">
    <div class="category-name">
      {name}
    </div>

    {#if mode === 'select' || mode === 'edit'}
      <div class="category-actions">
        <button onclick={onedit} aria-label="Edit {name} category">
          <Chip>
            <Pencil />
          </Chip>
        </button>

        {#if mode === 'select'}
          <button onclick={onclear} aria-label="Clear selected {name} tags">
            <Chip>
              <X />
            </Chip>
          </button>
        {/if}
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
  max-width: var(--block-size-m);

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
