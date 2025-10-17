<script lang="ts">
import { fade } from 'svelte/transition'
import Spinner from './Spinner.svelte'
</script>

<div class="preloader" out:fade={{ duration: 333 }}>
  <Spinner --spinner-color="var(--color-base-60)" />
  <div class="text">Gathering thoughts...</div>
</div>

<style lang="scss">
.preloader {
  position: fixed;
  top: 0;
  left: 0;
  z-index: 9876543210;
  width: 100vw;
  height: 100vh;

  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  gap: var(--padding-s);

  background-color: var(--color-base-20);
  color: var(--color-base-60);
}
</style>
