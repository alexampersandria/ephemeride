<script lang="ts">
import type { LabelProps } from '$lib/types/components/label'

let { children, for: forProp }: LabelProps = $props()
</script>

{#if forProp}
  <label for={forProp} class="label">
    {@render children()}
  </label>
{:else}
  <div class="label">
    {@render children()}
  </div>
{/if}

<style lang="scss">
.label {
  font-weight: 600;
}
</style>
