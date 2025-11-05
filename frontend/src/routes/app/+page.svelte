<script lang="ts">
import EntryPreview from '$lib/components/EntryPreview.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { currentDate } from '$lib/utils/log'

let userStore = useUserStore()
let dataStore = useDataStore()

let today = $derived.by(() => {
  return dataStore.getEntry(currentDate())
})
</script>

<div class="app-page home-page">
  <div class="container">
    {#if userStore.userDetails !== null}
      <div class="greeting">
        Welcome back {userStore.userDetails.name}
      </div>

      <div class="today">
        <div class="title">Today's Entry</div>
        <EntryPreview date={today ? today.date : currentDate()} entry={today} />
      </div>
    {/if}
  </div>
</div>

<style lang="scss">
.home-page {
  .greeting {
    font-size: var(--font-size-xl);
    margin-bottom: var(--padding-l);
    text-align: center;
  }

  .today {
    display: flex;
    flex-direction: column;
    gap: var(--padding-s);
  }
}
</style>
