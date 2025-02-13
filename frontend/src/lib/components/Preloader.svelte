<script lang="ts">
import { fade } from 'svelte/transition'
import Spinner from './Spinner.svelte'
</script>

<div class="preloader" transition:fade>
  <Spinner --spinner-size="2rem" --spinner-color="var(--color-base-60)" />
</div>

<style lang="scss">
.preloader {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;

  display: flex;
  justify-content: center;
  align-items: center;

  background-color: var(--color-base-05);
}
</style>
