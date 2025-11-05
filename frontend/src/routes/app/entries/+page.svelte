<script lang="ts">
import Spinner from '$lib/components/Spinner.svelte'
import { getEntries, type FetchEntriesOptions } from '$lib/utils/api'
import type { Entry } from '$lib/types/log'
import { ScrollText } from 'lucide-svelte'
import { onMount } from 'svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import Button from '$lib/components/Button.svelte'
import Input from '$lib/components/Input.svelte'
import Select from '$lib/components/Select.svelte'
import { goto } from '$app/navigation'
import EntryPreview from '$lib/components/EntryPreview.svelte'

let {
  data,
}: {
  data: {
    search?: string
  }
} = $props()

let search = $derived.by(() => {
  const urlSearchParams = new URLSearchParams(data?.search || '')

  return {
    from_date: urlSearchParams.get('from_date') || undefined,
    to_date: urlSearchParams.get('to_date') || undefined,
    tags: urlSearchParams.get('tags')?.split(',') || undefined,
    from_mood: urlSearchParams.get('from_mood') || undefined,
    to_mood: urlSearchParams.get('to_mood') || undefined,
    order: urlSearchParams.get('order') || 'date_desc',
  } as FetchEntriesOptions
})

let list: Entry[] = $state([])
let loading = $state(false)

let userStore = useUserStore()
let dataStore = useDataStore()

onMount(() => {
  options = { ...options, ...search }
  getData()
})

let options: FetchEntriesOptions = $state({
  from_date: undefined,
  to_date: undefined,
  tags: undefined,
  from_mood: undefined,
  to_mood: undefined,
  order: 'date_desc',
})

const reset = () => {
  options = {
    from_date: undefined,
    to_date: undefined,
    tags: undefined,
    from_mood: undefined,
    to_mood: undefined,
    order: 'date_desc',
  }
  getData()
}

const getData = async () => {
  if (userStore.sessionId) {
    const params = new URLSearchParams()
    if (options.from_date) params.append('from_date', options.from_date)
    if (options.to_date) params.append('to_date', options.to_date)
    if (options.tags) {
      const tagString = options.tags.join(',')
      if (tagString) {
        params.append('tags', tagString)
      }
    }
    if (options.from_mood) params.append('from_mood', `${options.from_mood}`)
    if (options.to_mood) params.append('to_mood', `${options.to_mood}`)
    if (options.order && options.order !== 'date_desc')
      params.append('order', options.order)

    goto(`/app/entries/?${params.toString()}`, { replaceState: true })

    loading = true
    const res = await getEntries(userStore.sessionId, options)
    if (res) {
      list = res
    }
    loading = false
  }
}

const toggleTag = (tagId: string) => {
  if (!options.tags) {
    options.tags = []
  }
  const index = options.tags.indexOf(tagId)
  if (index === -1) {
    options.tags.push(tagId)
  } else {
    options.tags.splice(index, 1)
  }
  getData()
}
</script>

<div class="app-page entries-page">
  <div class="container">
    <div class="app-page-title">
      <ScrollText />
      Entries (WIP)
    </div>

    <div class="filters">
      <div class="form-field inline">
        from:
        <input type="date" bind:value={options.from_date} onchange={getData} />
      </div>
      <div class="form-field inline">
        to:
        <input type="date" bind:value={options.to_date} onchange={getData} />
      </div>
      <div>
        tags:
        {#each dataStore.getTags() || [] as tag}
          <div class="form-field inline">
            <label for={tag.id}>
              {tag.category.name}/{tag.name}
            </label>
            <input
              type="checkbox"
              name={tag.id}
              id={tag.id}
              checked={options.tags?.includes(tag.id)}
              onchange={() => {
                toggleTag(tag.id)
              }} />
          </div>
        {/each}
      </div>
      <div class="form-field inline">
        from mood:
        <Input
          type="number"
          bind:value={options.from_mood}
          onchange={getData} />
      </div>
      <div class="form-field inline">
        to mood:
        <Input type="number" bind:value={options.to_mood} onchange={getData} />
      </div>

      <div class="form-field inline">
        order:
        <Select
          bind:value={options.order}
          options={[
            { label: 'Date Descending', value: 'date_desc' },
            { label: 'Date Ascending', value: 'date_asc' },
            { label: 'Mood Descending', value: 'mood_desc' },
            { label: 'Mood Ascending', value: 'mood_asc' },
          ]}
          onchange={getData} />
      </div>

      <div class="form-field">
        <Button onclick={reset}>Reset filters</Button>
      </div>
    </div>

    {#if loading}
      <Spinner />
    {:else if list.length === 0}
      <div class="dimmed">No entries found</div>
    {:else}
      <div class="entries">
        <div class="count">
          {list.length} entr{list.length === 1 ? 'y' : 'ies'} found
        </div>
        {#each list as entry}
          <div class="entry-item">
            <EntryPreview date={entry.date} {entry} />
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style lang="scss">
.entries-page {
  .filters {
    display: flex;
    flex-wrap: wrap;
    gap: var(--padding-s);
    margin-bottom: var(--padding-m);
  }

  .entries {
    display: flex;
    flex-wrap: wrap;
    gap: var(--padding-s);

    .count {
      font-size: var(--font-size-s);
      color: var(--text-muted);
      text-align: right;
      width: 100%;
      flex-basis: 100%;
      flex-grow: 1;
      flex-shrink: 0;
    }

    .entry-item {
      flex: 1 1 calc(50% - var(--padding-s));
      min-width: var(--block-size-xs);
      width: calc(100% / 3 - var(--padding-s));
    }
  }
}
</style>
