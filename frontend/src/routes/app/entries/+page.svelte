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
import type { PaginationObject } from '$lib/types/paginated'
import Checkbox from '$lib/components/Checkbox.svelte'
import Label from '$lib/components/Label.svelte'
import Message from '$lib/components/Message.svelte'
import { takeAtLeast } from '$lib/utils/takeAtLeast'
import Calendar from '$lib/components/Calendar.svelte'

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
    order: urlSearchParams.get('order') || undefined,
  } as FetchEntriesOptions
})

let list: Entry[] = $state([])
let error = $state<string | undefined>(undefined)
let pagination: PaginationObject = $state({
  limit: 0,
  offset: 0,
  total_count: 0,
})
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
  limit: 20,
  offset: 0,
})

const reset = () => {
  options = {
    from_date: undefined,
    to_date: undefined,
    tags: undefined,
    from_mood: undefined,
    to_mood: undefined,
    order: 'date_desc',
    limit: 20,
    offset: 0,
  }
  getData()
}

const getData = async (more = false) => {
  if (userStore.sessionId) {
    if (more) {
      options.offset = pagination.offset + pagination.limit
    } else {
      options.offset = 0
    }

    loading = true

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

    const res = await takeAtLeast(getEntries(userStore.sessionId, options))
    if (res) {
      if (more) {
        list = [...list, ...res.data]
      } else {
        list = res.data
      }
      pagination = res.pagination
    } else {
      list = []
      error = 'Failed to fetch entries'
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

const filtersApplied = $derived.by(() => {
  return (
    options.from_date !== undefined ||
    options.to_date !== undefined ||
    (options.tags !== undefined && options.tags.length > 0) ||
    options.from_mood !== undefined ||
    options.to_mood !== undefined ||
    (options.order !== undefined && options.order !== 'date_desc') ||
    (options.limit !== undefined && options.limit !== 20)
  )
})
</script>

<div class="app-page entries-page">
  <div class="container">
    <div class="app-page-title">
      <ScrollText />
      Entries (WIP)
    </div>

    <div class="filters">
      <div class="rangepicker">
        <Calendar
          mode="rangepicker"
          bind:from={options.from_date}
          bind:to={options.to_date}
          onchange={() => getData()} />
      </div>

      <div>
        tags:
        <div class="tag-options">
          {#each dataStore.getTags() || [] as tag}
            <div class="tag-option color-{tag.color}-text">
              <Checkbox
                id={tag.id}
                value={options.tags?.includes(tag.id) || false}
                onchange={() => toggleTag(tag.id)} />
              <Label for={tag.id}>
                {tag.category.name}/{tag.name}
              </Label>
            </div>
          {/each}
        </div>
      </div>
      <div class="form-field inline">
        <Label for="from-mood">from mood:</Label>
        <Input
          id="from-mood"
          type="number"
          bind:value={options.from_mood}
          onchange={() => getData()} />
      </div>
      <div class="form-field inline">
        <Label for="to-mood">to mood:</Label>
        <Input
          id="to-mood"
          type="number"
          bind:value={options.to_mood}
          onchange={() => getData()} />
      </div>

      <div class="form-field inline">
        limit:
        <Select
          bind:value={options.limit}
          options={[
            { label: '10', value: 10 },
            { label: '20', value: 20 },
            { label: '50', value: 50 },
            { label: '100', value: 100 },
            { label: 'all', value: 0 },
          ]}
          onchange={() => getData()} />
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
          onchange={() => getData()} />
      </div>

      <div class="form-field">
        <Button onclick={reset} disabled={!filtersApplied}>
          Reset filters
        </Button>
      </div>
    </div>

    {#if loading && options.offset === 0}
      <div class="loading">
        <Spinner />
      </div>
    {:else if error}
      <div class="flex-center">
        <Message type="error">{error}</Message>
      </div>
    {:else if list.length === 0}
      <div class="centered dimmed">No entries found</div>
    {:else}
      <div class="entries">
        <div class="count">
          showing {list.length} of {pagination.total_count} entries {filtersApplied
            ? 'matching filters'
            : ''}
        </div>
        {#each list as entry}
          <div class="entry-item">
            <EntryPreview date={entry.date} {entry} small />
          </div>
        {/each}
      </div>
      <div class="more">
        <Button
          fullwidth
          onclick={() => getData(true)}
          {loading}
          disabled={list.length >= pagination.total_count}>
          Load more
        </Button>
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

    .rangepicker {
      flex: 1 1 100%;
    }
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
      flex: 1 1 100%;
      max-width: 100%;
    }
  }

  .more {
    margin-top: var(--padding-m);
  }

  .tag-options {
    display: flex;
    flex-wrap: wrap;
    gap: var(--padding-s);
  }

  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: var(--padding-l) 0;
  }
}
</style>
