<script lang="ts">
import type { ButtonProps } from '$lib/types/components/button'
import Spinner from './Spinner.svelte'

let {
  children,
  type = 'secondary',
  loading,
  disabled,
  onclick,
}: ButtonProps = $props()

let clickHandler = () => {
  if (disabled) return
  if (loading) return
  if (!onclick) return

  onclick()
}
</script>

<button
  class="button {type}"
  class:loading
  class:disabled
  aria-busy={loading}
  {disabled}
  onclick={clickHandler}>
  <div class="button-content">
    {@render children()}
  </div>
  {#if loading}
    <div class="button-spinner">
      <Spinner />
    </div>
  {/if}
</button>

<style lang="scss">
.button {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: var(--button-padding);
  border-radius: var(--button-radius);
  color: var(--button-color);
  border: none;
  background-color: transparent;
  border: var(--border-width) solid var(--button-border);
  background-color: var(--button-background);

  &:active {
    transform: var(--click-transform);
  }

  .button-content {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--button-content-gap);
  }

  :global(.lucide-icon) {
    width: 1em;
    height: 1em;
  }

  .button-spinner {
    --spinner-size: 1rem;
    position: absolute;
  }

  &.loading {
    .button-content {
      opacity: 0;
    }
    .button-spinner {
      opacity: 1;
    }
  }

  &:not(:disabled, .loading) {
    cursor: pointer;

    &:hover {
      color: var(--button-color-hover);
      background-color: var(--button-background-hover);
      border-color: var(--button-border-hover);
    }

    &:active {
      color: var(--button-color-active);
      background-color: var(--button-background-active);
      border-color: var(--button-border-active);
    }
  }

  &.primary {
    color: var(--button-primary-color);
    --spinner-color: var(--button-primary-color);
    border-color: var(--button-primary-border);
    background-color: var(--button-primary-background);

    &:not(:disabled, .loading) {
      &:hover {
        color: var(--button-primary-color-hover);
        background-color: var(--button-primary-background-hover);
        border-color: var(--button-primary-border-hover);
      }

      &:active {
        color: var(--button-primary-color-active);
        background-color: var(--button-primary-background-active);
        border-color: var(--button-primary-border-active);
      }
    }
  }

  &.destructive {
    color: var(--button-destructive-color);
    --spinner-color: var(--button-destructive-color);
    border-color: var(--button-destructive-border);
    background-color: var(--button-destructive-background);

    &:not(:disabled, .loading) {
      &:hover {
        color: var(--button-destructive-color-hover);
        background-color: var(--button-destructive-background-hover);
        border-color: var(--button-destructive-border-hover);
      }

      &:active {
        color: var(--button-destructive-color-active);
        background-color: var(--button-destructive-background-active);
        border-color: var(--button-destructive-border-active);
      }
    }
  }

  &.ghost {
    background-color: transparent;
    border-color: transparent;

    &:not(:disabled, .loading) {
      &:hover {
        background-color: var(--button-background-hover);
        border-color: transparent;
      }

      &:active {
        background-color: var(--button-background-active);
        border-color: transparent;
      }
    }
  }

  &:disabled {
    color: var(--input-disabled-color) !important;
    background-color: var(--input-disabled-background) !important;
    border-color: var(--input-disabled-border) !important;
  }
}
</style>
