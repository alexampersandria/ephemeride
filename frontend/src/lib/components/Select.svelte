<script lang="ts">
import type { SelectProps } from '$lib/types/components/select'
import { evaluateInputState } from '$lib/utils/input'
import { ChevronDown } from 'lucide-svelte'
import { onMount } from 'svelte'

let {
  options,
  value = $bindable(),
  placeholder,
  disabled,
  inputstate = $bindable('untouched'),
  name,
  id,
  required,
  fullwidth,
  onchange: emitOnChange,
  oninput: emitOnInput,
  'aria-label': ariaLabel,
}: SelectProps = $props()

const onchange = (event: Event) => {
  const target = event.target as HTMLSelectElement
  value = target.value

  inputstate = evaluateInputState({
    value,
    inputstate,
    required,
  })

  if (emitOnChange) {
    emitOnChange(event)
  }
}

const oninput = (event: Event) => {
  if (emitOnInput) {
    emitOnInput(event)
  }
}

onMount(() => {
  if (!placeholder && !value) {
    value = options[0].value
  }
})
</script>

<div class="select-wrapper">
  <select
    class="select {placeholder && !value ? 'placeholder-selected' : ''}"
    {disabled}
    {name}
    {id}
    {required}
    class:fullwidth
    class:invalid={inputstate === 'invalid'}
    aria-invalid={inputstate === 'invalid'}
    aria-label={ariaLabel}
    {onchange}
    {oninput}>
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

  <div class="arrow">
    <ChevronDown />
  </div>
</div>

<style lang="scss">
.select-wrapper {
  display: inline-block;
  position: relative;

  .arrow {
    position: absolute;
    top: 50%;
    right: var(--select-arrow-right);
    transform: translateY(calc(-50% + 2px));
    pointer-events: none;
    color: var(--text-muted);
    transition: var(--interactive-transition);
  }

  &:has(.select:hover) .arrow {
    color: var(--text-normal);
  }

  .select {
    padding: var(--select-padding);
    border-radius: var(--select-radius);
    background-color: var(--select-background);
    border: var(--border-width) solid var(--select-border);
    appearance: none;

    &.fullwidth {
      width: 100%;
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
      box-shadow: 0 0 0 var(--state-shadow-width)
        var(--color-invalid-background);
      border-color: var(--color-invalid-border);
    }

    &.placeholder-selected {
      color: var(--select-placeholder-color);
    }
  }
}
</style>
