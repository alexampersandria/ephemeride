<script lang="ts">
import type { SelectProps } from '$lib/types/components/select'
import { onMount } from 'svelte'

let {
  options,
  value = $bindable(),
  placeholder,
  disabled,
  state = $bindable('untouched'),
  name,
  id,
  required,
  fullwidth,
}: SelectProps = $props()

const onchange = (event: Event) => {
  const target = event.target as HTMLSelectElement
  value = target.value

  if (state === 'untouched') {
    state = 'touched'
  }
}

onMount(() => {
  if (!placeholder && !value) {
    value = options[0].value
  }
})
</script>

<select
  class="select {placeholder && !value ? 'placeholder-selected' : ''}"
  {disabled}
  {name}
  {id}
  {required}
  class:fullwidth
  class:invalid={state === 'invalid'}
  aria-invalid={state === 'invalid'}
  {onchange}>
  {#if placeholder}
    <option class="select-option" value="" disabled selected={!value}>
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

  &.fullwidth {
    width: calc(100% - var(--input-padding) * 2);
  }

  &:not(:disabled) {
    cursor: pointer;

    &,
    option:not(:disabled) {
      color: var(--select-color);
    }

    &:hover {
      background-color: var(--select-background-hover);
    }

    &:focus {
      background-color: var(--select-background);
    }
  }

  &:disabled {
    color: var(--input-disabled-color);
    background-color: var(--input-disabled-background);
  }

  option:disabled {
    color: var(--input-disabled-color);
  }

  &.invalid {
    box-shadow: 0 0 0 var(--state-shadow-width) var(--color-invalid-background);
    border-color: var(--color-invalid-border);
  }

  &.placeholder-selected {
    color: var(--select-placeholder-color);
  }
}
</style>
