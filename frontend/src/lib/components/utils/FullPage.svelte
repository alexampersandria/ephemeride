<script>
import { teleport } from '$lib/actions/teleport'

let { children } = $props()
</script>

<div class="full-page" use:teleport={'#root'}>
  {#if children}
    {@render children()}
  {/if}
</div>

<style lang="scss">
.full-page {
  position: fixed;
  top: 0;
  left: 0;

  width: 100vw;
  height: 100vh;

  display: flex;
  justify-content: center;
  align-items: center;

  z-index: 1000;
}
</style>
