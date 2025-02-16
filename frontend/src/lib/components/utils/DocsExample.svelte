<script lang="ts">
import type { Snippet } from 'svelte'

const {
  children,
  gap,
  left,
}: {
  children: Snippet
  gap?: string
  left?: boolean
} = $props()
</script>

<div class="docs-example" class:left style:gap>
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

  &.left {
    justify-content: flex-start;
  }
}
</style>
