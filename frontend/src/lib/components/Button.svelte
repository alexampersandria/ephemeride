<script lang="ts">
// #TODO: Implement the Button component properly, just demo for Banner
import type { ButtonProps } from '$lib/types/button'

let { children, variant = 'primary' }: ButtonProps = $props()
</script>

<button class="button">
  {@render children()}
</button>

<style lang="scss">
.button {
  padding: var(--button-padding);
  border-radius: var(--button-radius);
  background-color: var(--background-primary);
  border: var(--border-width) solid var(--border-color);
  &:not(:disabled) {
    cursor: pointer;

    &:hover {
      background-color: var(--background-secondary);
    }
  }

  &:disabled {
    cursor: not-allowed;
    opacity: 1;
    color: var(--text-dimmed);
    background-color: var(--background-secondary);
  }
}
</style>
