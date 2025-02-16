<script lang="ts">
import type { SelectProps } from '$lib/types/select'

let {
  options,
  value = $bindable(),
  placeholder,
  disabled,
  state = $bindable('untouched'),
}: SelectProps = $props()

const onchange = (event: Event) => {
  const target = event.target as HTMLSelectElement
  value = target.value

  if (state === 'untouched') {
    state = 'touched'
  }
}
</script>

<select
  class="select"
  class:valid={state === 'valid'}
  class:invalid={state === 'invalid'}
  {disabled}
  aria-invalid={state === 'invalid'}
  {onchange}>
  {#if placeholder}
    <option
      class="select-option"
      value=""
      disabled
      aria-disabled="true"
      selected={!value}>
      {placeholder}
    </option>
  {/if}
  {#each options as option}
    <option
      class="select-option"
      value={option.value}
      selected={option.value === value}
      disabled={option.disabled}>
      {option.label}
    </option>
  {/each}
</select>

<style lang="scss">
.select {
  padding: var(--select-padding);
  border-radius: var(--select-radius);
  background-color: var(--select-background);
  border: var(--border-width) solid var(--select-border);

  &:not(:disabled) {
    cursor: pointer;

    &:hover {
      background-color: var(--select-background-hover);
    }
  }

  &:disabled {
    color: var(--input-disabled-color);
    background-color: var(--input-disabled-background);
  }

  &.invalid {
    box-shadow: 0 0 0 var(--state-shadow-width) var(--color-invalid-background);
    border-color: var(--color-invalid-border);
  }

  &.valid {
    box-shadow: 0 0 0 var(--state-shadow-width) var(--color-valid-background);
    border-color: var(--color-valid-border);
  }
}
</style>
