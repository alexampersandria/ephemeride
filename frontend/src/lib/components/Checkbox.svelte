<script lang="ts">
import type { CheckboxProps } from '$lib/types/components/checkbox'
import { Check } from 'lucide-svelte'

let {
  value = $bindable(false),
  disabled,
  inputstate = $bindable('untouched'),
  id,
  required,
  'aria-label': ariaLabel,
}: CheckboxProps = $props()

const onClick = () => {
  if (!disabled) {
    value = !value

    if (required && !value) {
      inputstate = 'invalid'
    } else {
      inputstate = 'touched'
    }
  }
}
</script>

<button
  role="checkbox"
  class="checkbox"
  class:checked={value}
  class:invalid={inputstate === 'invalid'}
  class:disabled
  aria-label={ariaLabel}
  aria-checked={value}
  {disabled}
  {id}
  onclick={onClick}>
  <div class="check">
    <Check />
  </div>
</button>

<style lang="scss">
.checkbox {
  vertical-align: middle;
  height: var(--checkbox-size);
  width: var(--checkbox-size);

  background-color: var(--checkbox-background-color);
  border: var(--border-width) solid var(--checkbox-border-color);
  border-radius: var(--radius-xs);

  display: inline-flex;
  align-items: center;
  justify-content: center;

  &:not(.disabled) {
    &:hover {
      background-color: var(--checkbox-background-color-hover);
    }

    &:active {
      background-color: var(--checkbox-background-color-active);
    }
  }

  .check {
    position: relative;
    top: 1px;
    font-size: var(--checkbox-checkmark-size);
    color: var(--checkbox-checkmark-color);

    transition:
      opacity var(--animation-length-s) var(--better-ease-out),
      transform var(--animation-length-s) var(--better-ease-out),
      filter var(--animation-length-m) var(--better-ease-out);
  }

  &:not(.checked) {
    .check {
      opacity: 0;
      transform: scale(0.8);
      filter: blur(2px);
    }
  }

  &.checked {
    background-color: var(--checkbox-checked-background-color);
    border-color: var(--checkbox-checked-border-color);

    &:not(.disabled) {
      &:hover {
        background-color: var(--checkbox-checked-background-color-hover);
      }

      &:active {
        background-color: var(--checkbox-checked-background-color-active);
      }
    }
  }

  &.invalid {
    box-shadow: 0 0 0 var(--state-shadow-width) var(--color-invalid-background);
    border-color: var(--color-invalid-border);
  }
}
</style>
