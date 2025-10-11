<script lang="ts">
import type { ModalProps } from '$lib/types/components/modal'
import { X } from 'lucide-svelte'
import Portal from 'svelte-portal'
import { fade } from 'svelte/transition'

let { children, open = $bindable(false), size = 's' }: ModalProps = $props()

const close = () => {
  open = false
}

$effect(() => {
  const onKeyDown = (event: KeyboardEvent) => {
    if (event.key === 'Escape' && open) {
      close()
    }
  }

  window.addEventListener('keydown', onKeyDown)

  return () => {
    window.removeEventListener('keydown', onKeyDown)
  }
})
</script>

<Portal target="#root">
  {#if open}
    <div
      in:fade={{ duration: 50 }}
      out:fade={{ duration: 50 }}
      class="modal-wrapper">
      <div class="modal-backdrop" onclick={close} role="presentation"></div>
      <div class="modal size-{size}" role="dialog" aria-modal="true">
        <div class="control">
          <button onclick={close} class="basic" aria-label="Close modal">
            <X />
          </button>
        </div>
        <div class="content">
          {@render children()}
        </div>
      </div>
    </div>
  {/if}
</Portal>

<style lang="scss">
.modal-wrapper {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100dvh;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  overscroll-behavior: contain;

  .modal-backdrop {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    backdrop-filter: var(--modal-filter);
    background-color: var(--modal-backdrop);

    &:after {
      background: var(--noise-asset);
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      opacity: 0.02;
    }
  }

  .modal {
    position: relative;
    z-index: 1;

    background-color: var(--modal-background);
    border-radius: var(--modal-radius);
    box-shadow: var(--modal-shadow);
    padding: var(--modal-padding);
    border: var(--border-width) solid var(--modal-border);
    max-height: 75vh;
    min-height: calc(var(--modal-padding) * 2 + 2lh);
    overflow: auto;

    display: flex;
    flex-direction: column;
    justify-content: center;

    &.size-xxs {
      width: var(--block-size-xxs);
    }
    &.size-xs {
      width: var(--block-size-xs);
    }
    &.size-s {
      width: var(--block-size-s);
    }
    &.size-m {
      width: var(--block-size-m);
    }
    &.size-l {
      width: var(--block-size-l);
    }
    &.size-xl {
      width: var(--block-size-xl);
    }

    .control {
      position: absolute;
      top: 0;
      right: 0;

      button {
        padding: calc(var(--modal-padding) / 1.675);
      }
    }
  }
}
</style>
