<script lang="ts">
import Button from '$lib/components/Button.svelte'
import Chip from '$lib/components/Chip.svelte'
import Input from '$lib/components/Input.svelte'
import Modal from '$lib/components/Modal.svelte'
import Select from '$lib/components/Select.svelte'
import type { CategoryProps } from '$lib/types/assemblies/category'
import { colors, type Color } from '$lib/types/color'
import type { InputState } from '$lib/types/input'
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

let addTag = $state({
  open: false,
  name: {
    value: '',
    inputstate: 'untouched' as InputState,
  },
  color: {
    value: '',
    inputstate: 'untouched' as InputState,
  },
})

const toggleAddTag = () => {
  addTag.open = !addTag.open

  if (!addTag.open) {
    resetAddTag()
  }
}

const validateAddTag = () => {
  if (!addTag.name.value) {
    addTag.name.inputstate = 'invalid'
  }

  if (!addTag.color.value) {
    addTag.color.inputstate = 'invalid'
  }

  if (
    tags.find(tag => tag.name.toLowerCase() === addTag.name.value.toLowerCase())
  ) {
    addTag.name.inputstate = 'invalid'
  }
}

const addNewTag = () => {
  validateAddTag()

  if (
    addTag.name.inputstate !== 'touched' ||
    addTag.color.inputstate !== 'touched'
  ) {
    return
  }

  tags.push({
    id: Date.now().toString(),
    name: addTag.name.value,
    color: addTag.color.value as Color,
  })

  resetAddTag()
}

$effect(() => {
  if (!addTag.open) {
    resetAddTag()
  }
})

const validAddTag = $derived.by(() => {
  return (
    addTag.name.inputstate !== 'invalid' &&
    addTag.color.inputstate !== 'invalid'
  )
})

const resetAddTag = () => {
  addTag.name.value = ''
  addTag.color.value = ''
  addTag.open = false
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
          <button onclick={toggleAddTag} aria-label="Add tag">
            <Chip>
              <Plus />
            </Chip>
          </button>
          <Modal bind:open={addTag.open}>
            <h4>Add tag</h4>
            <div class="add-tag">
              <Input
                fullwidth
                live
                required
                bind:value={addTag.name.value}
                bind:inputstate={addTag.name.inputstate}
                placeholder="Tag name" />
              <!-- #TODO: add color picker component -->
              <Select
                fullwidth
                required
                options={colors.map(color => ({ label: color, value: color }))}
                bind:value={addTag.color.value}
                bind:inputstate={addTag.color.inputstate}
                placeholder="Tag color" />
              <div class="add-tag-actions">
                <Button onclick={toggleAddTag}>Cancel</Button>
                <Button
                  type="primary"
                  onclick={addNewTag}
                  disabled={!validAddTag}>Add tag</Button>
              </div>
            </div>
          </Modal>
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

.add-tag {
  display: flex;
  flex-direction: column;
  gap: var(--padding-s);

  .add-tag-actions {
    display: flex;
    justify-content: space-between;
  }
}
</style>
