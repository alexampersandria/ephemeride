<script lang="ts">
import type { SelectOption } from '$lib/types/select'

let {
  options,
  value = $bindable(),
}: {
  options: SelectOption[]
  value?: SelectOption['value']
} = $props()

const onchange = (event: Event) => {
  const target = event.target as HTMLSelectElement
  value = target.value
}
</script>

<select class="select" {onchange}>
  {#each options as option}
    <option class="select-option" value={option.value} selected={option.value === value}>
      {option.label}
    </option>
  {/each}
</select>
