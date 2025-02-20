<script lang="ts">
import type { LabelProps } from '$lib/types/components/label'

let { children, for: forProp }: LabelProps = $props()
</script>

<label for={forProp} class="label">
  {@render children()}
</label>
