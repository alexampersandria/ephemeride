<script lang="ts">
import Button from '$lib/components/Button.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { ArrowLeft, Book, LogOut } from 'lucide-svelte'

let userStore = useUserStore()
</script>

<div class="container">
  <div class="app">
    <div class="links small muted">
      <a href="/"><ArrowLeft /> landing page</a>
      <a href="/docs"><Book /> documentation</a>
    </div>
    <div class="dog">🐶</div>

    <div class="user">
      <code
        ><pre>sessionId: {JSON.stringify(userStore.sessionId)}
userDetails: {JSON.stringify(userStore.userDetails, null, 2)}</pre></code>

      {#if userStore.sessionId !== null}
        <Button onclick={() => userStore.logOut()}>
          <LogOut />
          Log out
        </Button>
      {/if}
    </div>
  </div>
</div>

<style lang="scss">
.app {
  display: flex;
  flex-direction: column;
  gap: var(--padding-l);
  padding: var(--padding-l);

  .dog {
    font-size: 2rem;
    text-align: center;
  }

  .links {
    display: flex;
    justify-content: space-between;
  }
}
</style>
