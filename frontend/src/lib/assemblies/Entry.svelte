<script lang="ts">
import type { EntryProps } from '$lib/types/assemblies/entry'
import MoodInput from '$lib/components/MoodInput.svelte'
import Textarea from '$lib/components/Textarea.svelte'
import Category from './Category.svelte'
import Button from '$lib/components/Button.svelte'
import {
  FolderOpen,
  NotebookText,
  Pencil,
  PencilOff,
  Plus,
  Save,
  SaveOff,
  Signature,
  Smile,
  Trash,
} from 'lucide-svelte'
import { entryMaxLength } from '$lib/types/log'
import Alert from '$lib/components/Alert.svelte'
import Chip from '$lib/components/Chip.svelte'
import Modal from '$lib/components/Modal.svelte'
import Markdown from 'svelte-exmarkdown'
import type { MoodValue } from '$lib/types/components/moodinput'
import Input from '$lib/components/Input.svelte'
import type { Category as CategoryType, NewTag, Tag } from '$lib/types/log'
import type { InputState } from '$lib/types/input'
import Message from '$lib/components/Message.svelte'
import { onMount } from 'svelte'
import { currentDate, fullDate, sortCategories } from '$lib/utils/log'
import { diff } from 'deep-object-diff'
import { formatNumber } from '$lib/utils/numbers'

let {
  id = undefined,
  date = currentDate(),
  mode = $bindable('view'),
  categories = [],
  entry = '',
  mood = undefined,
  selectedTagIds = [],

  onCreate,
  onUpdate,
  onDelete,
  onAddTag,
  onEditTag,
  onRemoveTag,
  onAddCategory,
  onEditCategory,
  onDeleteCategory,
}: EntryProps = $props()

let inputState: InputState = $state('untouched')

let errors = $derived.by(() => {
  const errs: string[] = []
  if (inputState === 'untouched') {
    return errs
  }

  if (editModel.entry.length > entryMaxLength) {
    errs.push(
      `Entry text exceeds maximum length of ${formatNumber(entryMaxLength)} characters`,
    )
  }

  if (editModel.mood === undefined) {
    errs.push('Mood value is required')
  }

  return errs
})

let entryTextModal = $state(false)

let editModel = $state<{
  mood?: MoodValue
  entry: string
  // the editmodel categories no longer work, the edit model should only be for the entry itself
  selectedTagIds: string[]
}>({
  mood: mood,
  entry: '',
  selectedTagIds: [],
})

const resetEditModel = () => {
  inputState = 'untouched'
  editModel.mood = mood
  editModel.entry = entry
  editModel.selectedTagIds = selectedTagIds
  resetCategoryDetails()
}

const applyEditModel = async () => {
  mood = editModel.mood
  entry = editModel.entry
  selectedTagIds = editModel.selectedTagIds
}

const startEdit = () => {
  resetEditModel()
  mode = 'edit'
}

const saveChanges = async () => {
  inputState = 'touched'

  if (errors.length > 0) {
    return
  }

  await applyEditModel()

  if (mood !== undefined) {
    if (mode === 'create' && onCreate) {
      onCreate({
        date,
        entry,
        mood,
        selected_tags: selectedTagIds,
      })
    } else if (mode === 'edit' && id && onUpdate) {
      onUpdate({
        id,
        date,
        entry,
        mood,
        selected_tags: selectedTagIds,
      })
    }
  } else {
    console.error('Mood is undefined, cannot save entry')
    return
  }

  mode = 'view'
}

const cancelChanges = () => {
  resetEditModel()
  mode = 'view'
}

const deleteEntry = () => {
  if (id && onDelete) {
    onDelete(id)
  }
}

let categoryDetails: {
  mode: 'create' | 'edit'
  open: boolean
  id?: string
  name: {
    value: string
    inputstate: InputState
  }
  errors: string[]
} = $state({
  mode: 'create',
  open: false,
  id: undefined,
  name: {
    value: '',
    inputstate: 'untouched',
  },
  errors: [],
})

const validateCategoryDetails = () => {
  categoryDetails.errors = []
  categoryDetails.name.value = categoryDetails.name.value.trim()

  if (!categoryDetails.name.value) {
    categoryDetails.name.inputstate = 'invalid'
    categoryDetails.errors.push('Name is required')
  } else if (
    categories.find(
      c => c.name === categoryDetails.name.value && c.id !== categoryDetails.id,
    )
  ) {
    categoryDetails.name.inputstate = 'invalid'
    categoryDetails.errors.push('Name must be unique')
  }
}

const startAddCategory = () => {
  resetCategoryDetails()
  categoryDetails.mode = 'create'
  categoryDetails.open = true
  categoryDetails.id = Date.now().toString()
}

const startEditCategory = (category: CategoryType) => {
  resetCategoryDetails()
  categoryDetails.mode = 'edit'
  categoryDetails.name.value = category.name
  categoryDetails.open = true
  categoryDetails.id = category.id
}

const resetCategoryDetails = () => {
  categoryDetails.name.value = ''
  categoryDetails.name.inputstate = 'untouched'
  categoryDetails.open = false
  categoryDetails.errors = []
  categoryDetails.id = undefined
}

const submitAddCategory = () => {
  validateCategoryDetails()
  if (categoryDetails.name.inputstate === 'invalid') {
    return
  }

  if (onAddCategory) {
    onAddCategory({
      name: categoryDetails.name.value,
    })
  }

  resetCategoryDetails()
}

const submitEditCategory = () => {
  validateCategoryDetails()
  if (categoryDetails.name.inputstate === 'invalid' || !categoryDetails.id) {
    return
  }

  if (onEditCategory) {
    onEditCategory({
      id: categoryDetails.id,
      name: categoryDetails.name.value,
    })
  }

  resetCategoryDetails()
}

const deleteCategory = () => {
  if (!categoryDetails.id) return

  editModel.selectedTagIds = editModel.selectedTagIds.filter(tagId => {
    return categories.some(c => c.tags.some(t => t.id === tagId))
  })
  if (onDeleteCategory) {
    onDeleteCategory(categoryDetails.id)
  }

  resetCategoryDetails()
}

onMount(() => {
  resetEditModel()
})

const isEdited = $derived.by(() => {
  if (mode !== 'edit') {
    return false
  }

  const difference = diff(
    {
      mood: mood,
      selectedTagIds: selectedTagIds,
      entry: entry,
    },
    {
      mood: editModel.mood,
      selectedTagIds: editModel.selectedTagIds,
      entry: editModel.entry,
    },
  )

  return Object.keys(difference).length > 0
})

const categoryAddTag = async (tag: NewTag) => {
  if (onAddTag) {
    return onAddTag(tag)
  }
  return null
}

const categoryEditTag = async (tag: Tag) => {
  if (onEditTag) {
    return onEditTag(tag)
  }
  return null
}

const categoryRemoveTag = async (tagId: string) => {
  if (onRemoveTag) {
    return onRemoveTag(tagId)
  }
  return null
}
</script>

<div class="entry">
  <div class="entry-title">
    <div class="date">
      {fullDate(date)}
    </div>

    {#if isEdited}
      <Chip>
        <SaveOff />
        Unsaved changes
      </Chip>
    {/if}
  </div>

  <div class="entry-field mood-field">
    <div class="entry-field-title">
      <div class="inner">
        <Smile />
        Mood
      </div>
    </div>
    <MoodInput
      bind:value={editModel.mood}
      mode={mode === 'view' ? 'view' : 'edit'} />
  </div>

  <div class="entry-field categories-field">
    <div class="entry-field-title">
      <div class="inner">
        <FolderOpen />
        Categories
      </div>
    </div>

    <div class="categories">
      {#if categories.length === 0}
        <div class="muted">No categories</div>
      {/if}

      {#each sortCategories(categories) as category}
        <Category
          id={category.id}
          name={category.name}
          tags={category.tags}
          bind:selectedTagIds={editModel.selectedTagIds}
          onEditCategory={() => startEditCategory(category)}
          onAddTag={categoryAddTag}
          onEditTag={categoryEditTag}
          onRemoveTag={categoryRemoveTag}
          mode={mode === 'view' ? 'view' : 'select'} />
      {/each}
    </div>

    {#if mode === 'edit' || mode === 'create'}
      <div class="add-category">
        <Button type="ghost" fullwidth onclick={startAddCategory}>
          <Plus /> Add Category
        </Button>

        <Modal open={categoryDetails.open}>
          <div class="category-details">
            <div class="category-details-title">
              {#if categoryDetails.mode === 'create'}
                Add Category
              {:else if categoryDetails.mode === 'edit'}
                Edit Category
              {/if}
            </div>

            <div class="category-details-inputs">
              <Input
                required
                bind:value={categoryDetails.name.value}
                bind:inputstate={categoryDetails.name.inputstate}
                placeholder="Category name"
                fullwidth />

              {#if categoryDetails.errors.length}
                {#each categoryDetails.errors as error}
                  <Message size="small" type="error">
                    {error}
                  </Message>
                {/each}
              {/if}
            </div>

            <div class="category-details-actions">
              {#if categoryDetails.mode === 'create'}
                <Button onclick={submitAddCategory}>
                  <Plus />
                  Add
                </Button>
              {:else if categoryDetails.mode === 'edit'}
                <Button type="destructive" onclick={deleteCategory}>
                  <Trash />
                  Delete category
                </Button>

                <Button onclick={submitEditCategory}>
                  <Save />
                  Save changes
                </Button>
              {/if}
            </div>
          </div>
        </Modal>
      </div>
    {/if}
  </div>

  <div class="entry-field entry-field-text">
    <div class="entry-field-title">
      <div class="inner">
        <NotebookText />
        Entry
      </div>
      {#if mode === 'edit' || mode === 'create'}
        <button
          onclick={() => (entryTextModal = true)}
          aria-label="Show entry text formatting help">
          <Chip>
            <Signature />
          </Chip>
        </button>

        <Modal bind:open={entryTextModal}>
          <p>
            Entry text supports markdown formatting, it uses
            <a
              href="https://ssssota.github.io/svelte-exmarkdown/"
              target="_blank">
              svelte-exmarkdown
            </a>
            for formatting
          </p>
          <p>
            See general markdown documentation <a
              href="https://www.markdownguide.org/cheat-sheet/"
              target="_blank">
              here
            </a>
          </p>
        </Modal>
      {/if}
    </div>

    {#if mode === 'edit' || mode === 'create'}
      <div class="entry-textarea">
        <Textarea
          bind:value={editModel.entry}
          maxlength={entryMaxLength}
          placeholder="Write your thoughts here..."
          fullwidth />
      </div>
    {:else}
      <div class="entry-text">
        {#if entry}
          <div class="entry-text-content">
            <Markdown md={entry} />
          </div>
        {:else}
          <p class="muted">No entry</p>
        {/if}
      </div>
    {/if}
  </div>

  {#if errors.length}
    <Alert type="error" size="small">
      <b>Invalid entry</b>
      <ul class="muted">
        {#each errors as error}
          <li>{error}</li>
        {/each}
      </ul>
    </Alert>
  {/if}

  <div class="entry-actions">
    {#if mode === 'view'}
      {#if id}
        <Button type="destructive" onclick={() => deleteEntry()}>
          <Trash /> Delete entry
        </Button>
      {/if}

      <Button onclick={() => startEdit()}>
        <Pencil /> Edit
      </Button>
    {:else if mode === 'edit'}
      <Button onclick={() => cancelChanges()}>
        <PencilOff />
        Cancel
      </Button>
      <Button
        type="primary"
        onclick={() => saveChanges()}
        disabled={errors.length > 0}>
        <Save />
        Save changes
      </Button>
    {:else if mode === 'create'}
      <Button
        type="primary"
        onclick={() => saveChanges()}
        disabled={errors.length > 0}>
        <Plus />
        Create Entry
      </Button>
    {/if}
  </div>
</div>

<style lang="scss">
.entry {
  display: flex;
  flex-direction: column;
  gap: var(--padding-l);
  width: 100%;
  max-width: var(--block-size-s);

  .entry-title {
    font-weight: 600;
    font-size: var(--font-size-xl);
    display: flex;
    align-items: center;
    justify-content: space-between;

    .date {
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }

  .categories {
    display: flex;
    flex-direction: column;
    gap: var(--padding-m);
    padding-top: var(--padding-m);
    width: 100%;
  }

  .add-category {
    margin-top: var(--padding-m);
  }

  .entry-field {
    display: flex;
    flex-direction: column;

    &.mood-field {
      gap: var(--padding-s);
    }

    &.entry-field-text {
      .entry-text {
        :global(:last-of-type) {
          margin-bottom: 0;
        }
      }

      .entry-textarea {
        margin-top: var(--padding-s);
      }
    }

    .entry-field-title {
      font-weight: 600;
      font-size: var(--font-size-l);
      display: flex;
      justify-content: space-between;

      .inner {
        display: flex;
        align-items: center;
        gap: var(--padding-s);
      }
    }
  }

  .entry-actions {
    display: flex;
    justify-content: space-between;

    :global(:only-child) {
      margin-left: auto;
    }
  }
}

.category-details {
  display: flex;
  flex-direction: column;
  gap: var(--padding-s);

  .category-details-title {
    font-weight: 600;
    font-size: var(--font-size-xl);
  }

  .category-details-inputs {
    display: flex;
    gap: var(--padding-s);
    flex-direction: column;
    margin: var(--padding-m) 0;
  }

  .category-details-actions {
    display: flex;
    justify-content: space-between;

    :global(:only-child) {
      margin-left: auto;
    }
  }
}
</style>
