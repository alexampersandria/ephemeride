<script lang="ts">
import type { LabelProps } from '$lib/types/components/label'

let {
  children,
  for: forProp,
  weight = 'bold',
  size = 'medium',
}: LabelProps = $props()
</script>

{#if forProp}
  <label for={forProp} class="label {weight} {size}">
    {@render children()}
  </label>
{:else}
  <div class="label {weight} {size}">
    {@render children()}
  </div>
{/if}

<style lang="scss">
.label {
  &.bold {
    font-weight: 600;
  }

  &.small {
    font-size: var(--font-size-s);
  }

  &.medium {
    font-size: var(--font-size-m);
  }

  &.large {
    font-size: var(--font-size-l);
  }
}
</style>
