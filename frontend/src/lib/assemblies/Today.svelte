<script lang="ts">
import EntryPreview from '$lib/components/EntryPreview.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { currentDate } from '$lib/utils/log'
import { CalendarDays } from 'lucide-svelte'

let userStore = useUserStore()
let dataStore = useDataStore()

let today = $derived.by(() => {
  return dataStore.getEntry(currentDate())
})
</script>

{#if userStore.userDetails !== null}
  <div class="today">
    <div class="app-page-title">
      <CalendarDays />
      Today's Entry
    </div>
    <EntryPreview date={today ? today.date : currentDate()} entry={today} />
  </div>
{/if}

<style lang="scss">
.today {
  :global(.entry-preview) {
    background-color: color-mix(var(--background-primary), transparent 70%);
    border: var(--border-width) solid
      color-mix(var(--border-color), transparent 50%);
    &:hover {
      background-color: color-mix(var(--background-primary), transparent 30%);
      border: var(--border-width) solid
        color-mix(var(--border-color), transparent 30%);
    }
    transition: var(--interactive-transition);

    :global(.mood-null) {
      background-color: color-mix(
        var(--background-accent),
        transparent 70%
      ) !important;
      transition: var(--interactive-transition);
    }
  }
}
</style>
