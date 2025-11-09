<script lang="ts">
import { fade } from 'svelte/transition'
import Spinner from './Spinner.svelte'
</script>

<div class="preloader" out:fade={{ duration: 150, delay: 150 }}>
  <div class="gradient-overlay"></div>
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

  background-color: var(--color-base-10);
  color: var(--color-base-100);

  .gradient-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: linear-gradient(
      135deg,
      rgba(255, 182, 232, 0.125) 0%,
      rgba(173, 216, 230, 0.1) 25%,
      rgba(226, 236, 249, 0.07) 50%,
      rgba(240, 170, 245, 0.1) 75%,
      rgba(176, 184, 230, 0.125) 100%
    );
    pointer-events: none;

    animation: gradientDrift 3s ease infinite;
    animation-delay: -1s;
    background-size: 200% 200%;
  }

  .text {
    font-size: var(--font-size-s);
    font-weight: 500;
    animation: textFadeIn 250ms ease both;
    animation-delay: 300ms;
  }

  @keyframes gradientDrift {
    0% {
      background-position: 0% 50%;
    }
    50% {
      background-position: 100% 50%;
    }
    100% {
      background-position: 0% 50%;
    }
  }

  @keyframes textFadeIn {
    0% {
      opacity: 0;
    }
    100% {
      opacity: 0.5;
    }
  }
}
</style>
