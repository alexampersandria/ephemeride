<script lang="ts">
const { children } = $props()
</script>

<div class="docs-example">
  {@render children()}
</div>

<style lang="scss">
.docs-example {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: var(--padding-l);
  gap: var(--padding-m);
  border-radius: var(--radius-s);
  border: var(--border-width) solid var(--border-color);
}
</style>
