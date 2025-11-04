<script lang="ts">
import { useDataStore } from '$lib/store/dataStore.svelte'
import { Tag, Tags } from 'lucide-svelte'
import type { PageProps } from './$types'
import Message from '$lib/components/Message.svelte'

let { data }: PageProps = $props()

let dataStore = useDataStore()

let tag = $derived.by(() => {
  return dataStore.getTag(data.id)
})
</script>

<div class="tag-page">
  <div class="container">
    {#if tag}
      <div class="title">
        <Tag />
        {tag.name} (WIP)
      </div>
    {:else}
      <div class="no-tag">
        <Message type="error">Error: Tag not found</Message>
        <div class="muted small">
          If you believe this to be an error <a
            href="http://github.com/alexampersandria/ephemeride/issues/new"
            target="_blank"
            rel="noopener noreferrer">create an issue</a>
          <br />
          <a href="/app/tags"><Tags /> See list of all tags</a>
        </div>
      </div>
    {/if}
  </div>
</div>

<style lang="scss">
.tag-page {
  padding: var(--padding-l);

  .title {
    font-size: var(--font-size-l);
    display: flex;
    align-items: center;
    gap: var(--padding-s);
    margin-bottom: var(--padding-m);
  }

  .no-tag {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: var(--padding-s);
  }
}
</style>
