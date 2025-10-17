<script lang="ts">
import { page } from '$app/state'
import Message from '$lib/components/Message.svelte'
import { ArrowLeft } from 'lucide-svelte'
</script>

<div class="error">
  <div class="container">
    <Message type="error" size="small">
      Error {page.status}: {page.error?.message || 'Unknown error'}
    </Message>
    <br />
    <div class="muted small">
      If you believe this to be an error <a
        href="http://github.com/alexampersandria/ephemeride/issues/new"
        target="_blank"
        rel="noopener noreferrer">please report it</a
      >.
      <br />
      <a href="/">
        <ArrowLeft />
        Go back to the homepage
      </a>
    </div>
  </div>
</div>

<style lang="scss">
.error {
  height: 100vh;
  width: 100vw;
  padding: var(--padding-xl) 0;
}
</style>
