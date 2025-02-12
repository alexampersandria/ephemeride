<script lang="ts">
import type { SelectOption } from '$lib/types/select'

let {
  options,
  selected = $bindable(),
}: {
  options: SelectOption[]
  selected?: SelectOption['value']
} = $props()

const onchange = (event: Event) => {
  const target = event.target as HTMLSelectElement
  selected = target.value
}
</script>

<select class="select" {onchange}>
  {#each options as option}
    <option class="select-option" value={option.value}>
      {option.label}
    </option>
  {/each}
</select>
