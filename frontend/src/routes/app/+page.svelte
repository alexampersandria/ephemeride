<script lang="ts">
import { useUserStore } from '$lib/store/userStore.svelte'
import { currentDate } from '$lib/utils/log'

let userStore = useUserStore()
</script>

<div class="container">
  {#if userStore.userDetails !== null}
    <div class="hello">
      <div class="large name">
        Welcome back {userStore.userDetails.name}
      </div>
      <div class="small muted date">
        Today is {currentDate()}
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
.hello {
  padding: var(--padding-xl);
  text-align: center;
  display: flex;
  flex-direction: column;
  gap: var(--padding-s);
}
</style>
