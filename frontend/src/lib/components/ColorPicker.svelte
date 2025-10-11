<script lang="ts">
import type { ColorPickerProps } from '$lib/types/components/colorpicker'
import { colors as defaultColors } from '$lib/types/color'
import Chip from './Chip.svelte'

let {
  colors = [...defaultColors],
  value = $bindable(),
  onChange,
  fullwidth,
  inputstate = $bindable('untouched'),
}: ColorPickerProps = $props()

const onclick = (selectedColor: string) => {
  value = selectedColor
  inputstate = 'touched'
  if (onChange) {
    onChange(selectedColor)
  }
}
</script>

<div class="color-picker" class:fullwidth>
  {#each colors as option}
    <div class="option color-{option}">
      <button
        onclick={() => onclick(option)}
        aria-label={`Select color ${option}`}>
        <Chip
          outline={value === option}
          color={option}
          variant={value === option || !value ? 'solid' : 'subtle'}>
          &#10240;
        </Chip>
      </button>
    </div>
  {/each}
</div>

<style lang="scss">
.color-picker {
  display: flex;
  flex-wrap: nowrap;
  gap: var(--padding-xs);

  &.fullwidth {
    width: 100%;
    justify-content: space-between;
  }
}
</style>
