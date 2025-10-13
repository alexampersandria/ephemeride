<script lang="ts">
import type { EntryProps } from '$lib/types/assemblies/entry'
import MoodInput from '$lib/components/MoodInput.svelte'
import Textarea from '$lib/components/Textarea.svelte'
import Category from './Category.svelte'
import Button from '$lib/components/Button.svelte'
import {
  Folder,
  Notebook,
  Pencil,
  Plus,
  Save,
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
import type { Category as CategoryType, CategoryWithTags } from '$lib/types/log'
import type { InputState } from '$lib/types/input'
import Message from '$lib/components/Message.svelte'

let {
  date,
  mode = $bindable('view'),
  categories = $bindable([]),
  entry = $bindable(''),
  mood = $bindable(),
  selectedTagIds = $bindable([]),
}: EntryProps = $props()

let errors = $derived.by(() => {
  const errs: string[] = []

  if (entry.length > entryMaxLength) {
    errs.push(
      `Entry text exceeds maximum length of ${entryMaxLength} characters.`,
    )
  }

  return errs
})

let entryTextModal = $state(false)

let editModel = $state<{
  mood?: MoodValue
  entry: string
  categories: CategoryWithTags[]
  selectedTagIds: string[]
}>({
  mood: undefined,
  entry: '',
  categories: [],
  selectedTagIds: [],
})

const resetEditModel = () => {
  editModel.mood = mood
  editModel.entry = JSON.parse(JSON.stringify(entry))
  editModel.categories = JSON.parse(JSON.stringify(categories))
  editModel.selectedTagIds = JSON.parse(JSON.stringify(selectedTagIds))
  resetCategoryDetails()
}

const applyEditModel = () => {
  if (editModel.mood !== undefined) {
    mood = editModel.mood
  }
  entry = editModel.entry
  categories = editModel.categories
  selectedTagIds = editModel.selectedTagIds
}

const startEdit = () => {
  resetEditModel()
  mode = 'edit'
}

const saveChanges = () => {
  if (errors.length > 0) {
    return
  }

  applyEditModel()
  mode = 'view'
}

const cancelChanges = () => {
  resetEditModel()
  mode = 'view'
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
    editModel.categories.find(
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

  editModel.categories.push({
    id: Date.now().toString(),
    name: categoryDetails.name.value,
    tags: [],
  })

  resetCategoryDetails()
}

const submitEditCategory = () => {
  validateCategoryDetails()

  if (categoryDetails.name.inputstate === 'invalid') {
    return
  }

  editModel.categories = editModel.categories.map(c => {
    if (c.id === categoryDetails.id) {
      return {
        ...c,
        name: categoryDetails.name.value,
      }
    }
    return c
  })

  resetCategoryDetails()
}

const deleteCategory = () => {
  if (!categoryDetails.id) return

  editModel.categories = editModel.categories.filter(
    c => c.id !== categoryDetails.id,
  )
  editModel.selectedTagIds = editModel.selectedTagIds.filter(tagId => {
    return editModel.categories.some(c => c.tags.some(t => t.id === tagId))
  })

  resetCategoryDetails()
}
</script>

<div class="entry">
  <div class="entry-title">
    Entry for {date || 'unknown date'}
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

  <div class="entry-field entry-field-text">
    <div class="entry-field-title">
      <div class="inner">
        <Notebook />
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
          <Markdown md={entry} />
        {:else}
          <p class="muted">No entry</p>
        {/if}
      </div>
    {/if}
  </div>

  <div class="entry-field categories-field">
    <div class="entry-field-title">
      <div class="inner">
        <Folder />
        Categories
      </div>
    </div>

    <div class="categories">
      {#if editModel.categories.length === 0}
        <div class="muted">No categories</div>
      {/if}

      {#each editModel.categories as category}
        <Category
          name={category.name}
          bind:tags={category.tags}
          bind:selectedTagIds={editModel.selectedTagIds}
          oneditcategory={() => startEditCategory(category)}
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

  {#if errors.length}
    <Alert type="error" size="small">
      <b>Invalid entry</b>
      <ul class="plain">
        {#each errors as error}
          <li>{error}</li>
        {/each}
      </ul>
    </Alert>
  {/if}

  <div class="entry-actions">
    {#if mode === 'view'}
      <Button onclick={() => startEdit()}>
        <Pencil /> Edit
      </Button>
    {:else if mode === 'edit' || mode === 'create'}
      <Button onclick={() => cancelChanges()}>Cancel</Button>
      <Button
        type="primary"
        onclick={() => saveChanges()}
        disabled={errors.length > 0}>Save</Button>
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
  }

  .categories {
    display: flex;
    flex-direction: column;
    gap: var(--padding-m);
    padding: var(--padding-m) 0;
    width: 100%;
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
