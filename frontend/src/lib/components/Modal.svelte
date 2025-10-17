<script lang="ts">
import type { ModalProps } from '$lib/types/components/modal'
import { firstFocusable } from '$lib/utils/focus'
import { X } from 'lucide-svelte'
import Portal from 'svelte-portal'

let { children, open = $bindable(false), size = 's' }: ModalProps = $props()

const close = () => {
  open = false
}

let shown = $state(false)

let previousFocusTarget = $state<HTMLElement | null>(null)
let firstFocusableElement = $state<HTMLElement | null>(null)
let modal = $state<HTMLElement | null>(null)

$effect(() => {
  if (open) {
    shown = true
    if (modal && !previousFocusTarget && !firstFocusableElement) {
      previousFocusTarget = document.activeElement as HTMLElement
      firstFocusableElement = firstFocusable(modal.querySelector('.content')!)
      firstFocusableElement?.focus()
    }
  } else {
    previousFocusTarget?.focus()
    previousFocusTarget = null
    firstFocusableElement = null
    // delay unmounting for animation
    setTimeout(() => {
      if (!open) {
        shown = false
      }
    }, 333)
  }
})

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
  {#if shown}
    <div class="modal-wrapper" class:closed={!open} bind:this={modal}>
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
  padding: var(--padding-m);

  @keyframes openModal {
    from {
      opacity: 0;
      transform: translateY(0.66rem);
      filter: blur(4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
      filter: blur(0);
    }
  }

  @keyframes closeModal {
    from {
      opacity: 1;
      filter: blur(0);
    }
    to {
      opacity: 0;
      filter: blur(4px);
    }
  }

  @keyframes openBackdrop {
    from {
      backdrop-filter: none;
      background-color: transparent;
    }
    to {
      backdrop-filter: var(--modal-filter);
      background-color: var(--modal-backdrop);
    }
  }

  @keyframes closeBackdrop {
    from {
      backdrop-filter: var(--modal-filter);
      background-color: var(--modal-backdrop);
    }
    to {
      backdrop-filter: none;
      background-color: transparent;
    }
  }

  @keyframes openBackdropAfter {
    from {
      opacity: 0;
    }
    to {
      opacity: 0.02;
    }
  }

  @keyframes closeBackdropAfter {
    from {
      opacity: 0.02;
    }
    to {
      opacity: 0;
    }
  }

  &.closed {
    pointer-events: none;
    .modal-backdrop {
      animation: closeBackdrop 0.3s ease-out;
      animation-fill-mode: forwards;

      &:after {
        animation: closeBackdropAfter 0.15s ease-out;
        animation-delay: 0;
        animation-fill-mode: forwards;
      }
    }

    .modal {
      animation: closeModal 0.15s ease-out;
      animation-delay: 0;
      animation-fill-mode: forwards;
    }
  }

  .modal-backdrop {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    backdrop-filter: var(--modal-filter);
    background-color: var(--modal-backdrop);

    animation: openBackdrop 0.3s ease-out;
    animation-fill-mode: backwards;

    &:after {
      background: var(--noise-asset);
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      opacity: 0.02;

      animation: openBackdropAfter 0.5s ease-out;
      animation-delay: 0.15s;
      animation-fill-mode: backwards;
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

    animation: openModal 0.1s ease-out;
    animation-delay: 0.05s;
    animation-fill-mode: backwards;

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
