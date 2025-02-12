<script lang="ts">
const { children } = $props()
</script>

<div class="docs-example">
  {@render children()}
</div>

<style lang="scss">
.docs-example {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  padding: var(--padding-l);
  border-radius: var(--radius-s);
  border: var(--border-width) solid var(--background-accent);
}
</style>
