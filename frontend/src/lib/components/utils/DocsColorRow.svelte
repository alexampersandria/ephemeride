<script lang="ts">
import { browser } from '$app/environment'
import { cssVariables } from '$lib/utils/cssVariables'
import DocsColor from './DocsColor.svelte'

let { color }: { color: string } = $props()
let regex = $derived(new RegExp(`^--color-${color}-`))

const cssVariable = $derived.by(() => {
  if (browser) {
    return cssVariables(regex)
  } else {
    return []
  }
})
</script>

<div class="docs-color-row">
  {#each cssVariable as color}
    <DocsColor {color} />
  {/each}
</div>

<style lang="scss">
.docs-color-row {
  display: flex;
  height: 4rem;
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--radius-m);
  overflow: hidden;
}
</style>
