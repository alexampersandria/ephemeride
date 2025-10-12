<script lang="ts">
import type { EntryProps } from '$lib/types/assemblies/entry'
import MoodInput from '$lib/components/MoodInput.svelte'
import Textarea from '$lib/components/Textarea.svelte'
import Category from './Category.svelte'
import Button from '$lib/components/Button.svelte'
import { Pencil } from 'lucide-svelte'
import { entryMaxLength } from '$lib/types/log'
import Alert from '$lib/components/Alert.svelte'

let {
  date,
  mode = 'view',
  categories = [],
  entry = '',
  mood = $bindable(),
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
</script>

<div class="entry">
  <div class="entry-title">
    Entry for {date || 'unknown date'}
  </div>

  <div class="entry-field mood-field">
    <div class="entry-field-title">Mood</div>
    <MoodInput bind:value={mood} mode={mode === 'view' ? 'view' : 'edit'} />
  </div>

  {#if categories.length}
    <div class="entry-field categories-field">
      <div class="entry-field-title">Categories</div>

      <div class="categories">
        {#each categories as category}
          <Category name={category.name} tags={category.tags} />
        {/each}
      </div>
    </div>
  {/if}

  <div class="entry-field entry-field-text">
    <div class="entry-field-title">Entry</div>

    {#if mode === 'edit' || mode === 'create'}
      <div class="entry-textarea">
        <Textarea
          bind:value={entry}
          maxlength={entryMaxLength}
          placeholder="Write your thoughts here..."
          fullwidth />
      </div>
    {:else}
      <div class="entry-text">
        {#if entry}
          <p>
            {entry}
          </p>
        {:else}
          <p class="muted">No entry text.</p>
        {/if}
      </div>
    {/if}
  </div>

  {#if errors.length}
    <Alert type="error" size="small">
      <b>Invalid entry</b>
      <ul>
        {#each errors as error}
          <li>{error}</li>
        {/each}
      </ul>
    </Alert>
  {/if}

  <div class="entry-actions">
    {#if mode === 'view'}
      <Button onclick={() => (mode = 'edit')}>
        <Pencil /> Edit
      </Button>
    {:else if mode === 'edit' || mode === 'create'}
      <Button onclick={() => (mode = 'view')}>Cancel</Button>
      <Button
        type="primary"
        onclick={() => (mode = 'view')}
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
    width: 100%;
  }

  .entry-field {
    display: flex;
    flex-direction: column;

    &.mood-field {
      gap: var(--padding-s);
    }

    &.entry-field-text {
      .entry-textarea {
        margin-top: var(--padding-s);
      }
    }

    .entry-field-title {
      font-weight: 600;
      font-size: var(--font-size-l);
      overflow: hidden;
      text-overflow: ellipsis;
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
</style>
