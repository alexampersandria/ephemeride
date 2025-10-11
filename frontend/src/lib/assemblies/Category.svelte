<script lang="ts">
import Alert from '$lib/components/Alert.svelte'
import Button from '$lib/components/Button.svelte'
import Chip from '$lib/components/Chip.svelte'
import Input from '$lib/components/Input.svelte'
import Modal from '$lib/components/Modal.svelte'
import Select from '$lib/components/Select.svelte'
import type { CategoryProps } from '$lib/types/assemblies/category'
import { colors, type Color } from '$lib/types/color'
import type { InputState } from '$lib/types/input'
import { Pencil, PencilOff, Plus, TagIcon, X } from 'lucide-svelte'
import type { Tag } from '$lib/types/log'

let {
  name,
  tags = $bindable([]),
  selectedTagIds = $bindable([]),
  mode = 'view',
  onaddtag,
  onremovetag,
  onselecttag,
  onedittag,
}: CategoryProps = $props()

const clickTag = (tag: Tag) => {
  if (mode === 'select') {
    if (selectedTagIds.includes(tag.id)) {
      selectedTagIds = selectedTagIds.filter(id => id !== tag.id)
    } else {
      selectedTagIds = [...selectedTagIds, tag.id]
    }

    if (onselecttag) {
      onselecttag(tag, selectedTagIds.includes(tag.id))
    }
  } else if (mode === 'edit') {
    openEditTag(tag.id)
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

let tagDetails: {
  mode: 'add' | 'edit'
  open: boolean
  id?: string
  name: {
    value?: string
    inputstate: InputState
  }
  color: {
    value?: Color
    inputstate: InputState
  }
  errors: string[]
} = $state({
  mode: 'add',
  open: false,
  id: undefined,
  name: {
    value: undefined,
    inputstate: 'untouched',
  },
  color: {
    value: undefined,
    inputstate: 'untouched',
  },
  errors: [] as string[],
})

const openAddTag = () => {
  tagDetails.open = true
  tagDetails.mode = 'add'
}

const openEditTag = (tagId: string) => {
  const tag = tags.find(tag => tag.id === tagId)
  if (tag) {
    tagDetails.mode = 'edit'
    tagDetails.id = tag.id
    tagDetails.name.value = tag.name
    tagDetails.name.inputstate = 'touched'
    tagDetails.color.value = tag.color
    tagDetails.color.inputstate = 'touched'
    tagDetails.open = true
  }
}

const closeTagDetails = () => {
  tagDetails.open = false
  resetTagDetails()
}

const validateAddTag = (requireUntouched = true) => {
  tagDetails.errors = []

  if (
    !tagDetails.name.value &&
    (tagDetails.name.inputstate !== 'untouched' || !requireUntouched)
  ) {
    tagDetails.name.inputstate = 'invalid'
    tagDetails.errors.push('Name is required')
  }

  if (
    !tagDetails.color.value &&
    (tagDetails.color.inputstate !== 'untouched' || !requireUntouched)
  ) {
    tagDetails.color.inputstate = 'invalid'
    tagDetails.errors.push('Color is required')
  }

  if (
    tags.find(
      tag =>
        tag.name.toLowerCase() === tagDetails.name.value?.toLowerCase() &&
        tag.id !== tagDetails.id,
    )
  ) {
    tagDetails.name.inputstate = 'invalid'
    tagDetails.errors.push('Tag name must be unique')
  }
}

const submitAddTag = () => {
  validateAddTag(false)

  if (
    tagDetails.name.inputstate !== 'touched' ||
    tagDetails.color.inputstate !== 'touched' ||
    !tagDetails.color.value ||
    !tagDetails.name.value
  ) {
    return
  }

  const newTag = {
    name: tagDetails.name.value,
    color: tagDetails.color.value as Color,
  }

  // timestamp, await ID from backend #TODO
  tags.push({
    id: Date.now().toString(),
    ...newTag,
  })

  if (onaddtag) {
    onaddtag(newTag)
  }

  resetTagDetails()
}

const submitEditTag = () => {
  validateAddTag(false)

  if (
    tagDetails.name.inputstate !== 'touched' ||
    tagDetails.color.inputstate !== 'touched' ||
    !tagDetails.color.value ||
    !tagDetails.name.value ||
    !tagDetails.id
  ) {
    return
  }

  const editedTag = {
    id: tagDetails.id,
    name: tagDetails.name.value,
    color: tagDetails.color.value as Color,
  }

  const tagIndex = tags.findIndex(tag => tag.id === tagDetails.id)
  if (tagIndex !== -1) {
    tags[tagIndex] = editedTag

    if (onedittag) {
      onedittag(editedTag)
    }
  }

  resetTagDetails()
}

$effect(() => {
  if (!tagDetails.open) {
    resetTagDetails()
  }
})

const resetTagDetails = () => {
  tagDetails.id = undefined
  tagDetails.name.value = undefined
  tagDetails.name.inputstate = 'untouched'
  tagDetails.color.value = undefined
  tagDetails.color.inputstate = 'untouched'
  tagDetails.errors = []
  tagDetails.open = false
}

const _removeTag = (tagId: string) => {
  if (mode === 'edit') {
    tags = tags.filter(tag => tag.id !== tagId)
    selectedTagIds = selectedTagIds.filter(id => id !== tagId)
    if (onremovetag) {
      onremovetag(tagId)
    }
  }
}

const validAddTag = $derived.by(() => {
  return (
    tagDetails.name.inputstate !== 'invalid' &&
    tagDetails.color.inputstate !== 'invalid'
  )
})
</script>

<div class="category">
  <div class="category-info">
    <div class="category-name">
      {name}
    </div>

    {#if mode === 'select' || mode === 'edit'}
      <div class="category-actions">
        {#if mode === 'edit'}
          <button onclick={openAddTag} aria-label="Add tag">
            <Chip>
              <Plus />
            </Chip>
          </button>

          <Modal bind:open={tagDetails.open}>
            <h4>Add tag</h4>
            <div class="add-tag">
              <Input
                fullwidth
                live
                required
                bind:value={tagDetails.name.value}
                bind:inputstate={tagDetails.name.inputstate}
                onchange={() => {
                  validateAddTag()
                }}
                placeholder="Tag name" />
              <!-- #TODO: add color picker component -->
              <Select
                fullwidth
                required
                options={colors.map(color => ({ label: color, value: color }))}
                bind:value={tagDetails.color.value}
                bind:inputstate={tagDetails.color.inputstate}
                onchange={() => {
                  validateAddTag()
                }}
                placeholder="Tag color" />

              {#if tagDetails.errors.length > 0}
                <Alert type="error" size="small">
                  <b>Invalid tag</b>
                  <ul>
                    {#each tagDetails.errors as error}
                      <li>{error}</li>
                    {/each}
                  </ul>
                </Alert>
              {/if}

              <div class="add-tag-actions">
                <Button onclick={closeTagDetails}>Cancel</Button>
                {#if tagDetails.mode === 'add'}
                  <Button
                    type="primary"
                    onclick={submitAddTag}
                    disabled={!validAddTag}>
                    Add tag
                  </Button>
                {:else if tagDetails.mode === 'edit'}
                  <Button
                    type="primary"
                    onclick={submitEditTag}
                    disabled={!validAddTag}>
                    Save changes
                  </Button>
                {/if}
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
        <TagIcon />
        No tags
      </div>
    {:else}
      {#each tags as tag}
        <button
          class="plain"
          onclick={() => clickTag(tag)}
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
