<script lang="ts">
import type { Snippet } from 'svelte'

const {
  children,
  gap,
  left,
  column,
}: {
  children: Snippet
  gap?: string
  left?: boolean
  column?: boolean
} = $props()
</script>

<div class="docs-example" class:left style:gap class:column>
  {@render children()}
</div>

<style lang="scss">
.docs-example {
  flex-wrap: wrap;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: var(--padding-l);
  gap: var(--padding-m);
  border-radius: var(--radius-s);
  border: var(--border-width) solid var(--border-color);
  overflow-x: auto;

  &.left {
    justify-content: flex-start;
    align-items: flex-start;
  }

  &.column {
    flex-direction: column;
  }
}
</style>
