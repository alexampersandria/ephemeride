<script lang="ts">
import type { Snippet } from 'svelte'

const {
  children,
  label,
}: {
  children: Snippet
  label?: string
} = $props()
</script>

<div class="docs-example" class:label>
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
}
</style>
