<script lang="ts">
import type { CssVariable } from '$lib/utils/cssVariables'

let { color }: { color: CssVariable } = $props()
</script>

<div class="docs-color" style="background-color: var({color.name}"></div>

<style lang="scss">
.docs-color {
  flex: 1;
  height: 100%;
}
</style>
