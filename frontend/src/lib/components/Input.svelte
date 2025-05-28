<script lang="ts">
import type { InputProps } from '$lib/types/components/input'
import { evaluateInputState } from '$lib/utils/input'

let {
  type = 'text',
  value = $bindable(),
  placeholder,
  disabled,
  inputstate = $bindable('untouched'),
  fullwidth,
  live,
  validation,
  name,
  id,
  required,
}: InputProps = $props()

const onchange = (event: Event) => {
  const target = event.target as HTMLSelectElement
  value = target.value

  inputstate = evaluateInputState({
    value,
    inputstate,
    validation,
    required,
  })
}

const oninput = (event: Event) => {
  if (live) {
    onchange(event)
  }
}
</script>

<input
  class="input"
  {type}
  {value}
  {placeholder}
  {disabled}
  {name}
  {id}
  {required}
  class:fullwidth
  class:invalid={inputstate === 'invalid'}
  aria-invalid={inputstate === 'invalid'}
  {onchange}
  {oninput} />

<style lang="scss">
.input {
  padding: var(--input-padding);
  border-radius: var(--input-radius);
  background-color: var(--input-background);
  border: var(--border-width) solid var(--input-border);

  &.fullwidth {
    width: 100%;
  }

  &:not(:disabled) {
    &:hover {
      background-color: var(--input-background-hover);
    }

    &:focus {
      background-color: var(--input-background);
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
}
</style>
