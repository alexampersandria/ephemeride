<script lang="ts">
import Button from '$lib/components/Button.svelte'
import Input from '$lib/components/Input.svelte'
import Label from '$lib/components/Label.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import type { MoodValue } from '$lib/types/components/moodinput'

let dataStore = useDataStore()

let log: string[] = $state([])
let i = $state(0)
let date = $state(new Date())
let limit = $state(10)

const generateData = async () => {
  console.log('Generating  data...')

  i = 0
  log = []
  let d = new Date(date)
  const tags = dataStore.getTags() || []

  while (i < limit) {
    const dateStr = d.toISOString().split('T')[0]

    let randomTag = tags[Math.floor(Math.random() * tags.length)]

    const entry = await dataStore.createEntry({
      date: dateStr,
      mood: (Math.floor(Math.random() * 5) + 1) as MoodValue,
      selected_tags: [randomTag.id],
    })

    if (entry) {
      log.push(`Created entry for ${dateStr}`)
    } else {
      log.push(`❌ Failed to create entry for ${dateStr}`)
    }
    d.setDate(d.getDate() + 1)
    i++
  }
}
</script>

<div class="app-page debug-page">
  <div class="container">
    <div class="form-field inline">
      <Label>limit</Label>
      <Input bind:value={limit} type="number" />
    </div>
    <div class="form-field inline">
      <Label>start date</Label>
      <input type="date" bind:value={date} />
    </div>
    <Button onclick={generateData}>Generate data</Button>
    <div class="i">
      i: {i}
    </div>
    <div class="log">
      {#each log as statement}
        <div class="statement">{statement}</div>
      {/each}
    </div>
  </div>
</div>

<style lang="scss">
.debug-page {
  .log {
    height: 400px;
    overflow-y: auto;
    border: 1px solid var(--border-color);
    padding: var(--spacing-m);
  }
}
</style>
