<script lang="ts">
import { fade } from 'svelte/transition'
import Spinner from './Spinner.svelte'
</script>

<div class="preloader" transition:fade>
  <Spinner --spinner-color="var(--color-base-60)" />
  <div class="shimmer">Gathering thoughts...</div>
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

  background-color: var(--color-base-00);

  .shimmer {
    // linear-gradient(to right, rgb(85, 205, 252), rgb(179, 157, 233), rgb(247, 168, 184), rgb(246, 216, 221), rgb(255, 255, 255) 45%, rgb(255, 255, 255), rgb(255, 255, 255) 55%, rgb(246, 216, 221), rgb(247, 168, 184), rgb(179, 157, 233), rgb(85, 205, 252))
    background: linear-gradient(
      0.33turn,
      var(--color-base-60) 30%,
      rgb(255, 255, 255) 34%,
      rgb(252, 133, 167) 38%,
      rgb(99, 210, 253) 42%,
      rgb(120, 79, 23),
      rgb(181, 63, 27),
      rgb(237, 34, 36),
      rgb(249, 150, 33),
      rgb(245, 193, 30),
      rgb(241, 235, 27),
      rgb(99, 199, 32),
      rgb(71, 96, 195),
      rgb(188, 49, 137),
      var(--color-base-60) 70%
    );
    font-weight: 500;
    background-size: 100rem;
    //background-position: 1rem 1rem;
    background-repeat: no-repeat;
    background-clip: text;
    -webkit-background-clip: text;
    color: transparent;

    @keyframes scrollBackground {
      0% {
        background-position: 0 0;
      }
      100% {
        background-position: 100% 0;
      }
    }

    animation: scrollBackground 4s cubic-bezier(0.55, 0.07, 0.34, 0.89) infinite;
  }
}
</style>
