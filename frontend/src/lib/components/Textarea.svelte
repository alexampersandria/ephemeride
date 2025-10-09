<script lang="ts">
import type { TextareaProps } from '$lib/types/components/textarea'
import { onMount } from 'svelte'

let {
  value = $bindable(),
  placeholder,
  disabled,
  name,
  id,
  required,
  maxlength,
  fullwidth,
  inputstate = $bindable('untouched'),
}: TextareaProps = $props()

let exceedsMaxLength = $derived.by(() => {
  if (value && maxlength) {
    return value.length > maxlength
  } else {
    return false
  }
})

const oninput = () => {
  evaluateInputState()
}

const evaluateInputState = () => {
  if (value) {
    if (maxlength) {
      if (value.length > maxlength) {
        inputstate = 'invalid'
      } else {
        inputstate = 'touched'
      }
    } else {
      inputstate = 'touched'
    }
  } else if (required) {
    inputstate = 'invalid'
  } else {
    inputstate = 'touched'
  }
}

onMount(() => {
  evaluateInputState()
})
</script>

<div class="textarea-wrapper" class:fullwidth>
  <textarea
    class="textarea"
    class:invalid={inputstate === 'invalid'}
    bind:value
    {placeholder}
    {disabled}
    {name}
    {id}
    {required}
    {oninput}>
  </textarea>

  {#if maxlength}
    <div
      class="textarea-maxlength"
      class:error={exceedsMaxLength}
      aria-live="polite"
      aria-atomic="true">
      {value ? value.length : 0} / {maxlength}
    </div>
  {/if}
</div>

<style lang="scss">
.textarea-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;

  .textarea {
    padding: var(--input-padding);
    border-radius: var(--input-radius);
    background-color: var(--input-background);
    border: var(--border-width) solid var(--input-border);
    resize: vertical;

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
      box-shadow: 0 0 0 var(--state-shadow-width)
        var(--color-invalid-background);
      border-color: var(--color-invalid-border);
    }
  }

  &.fullwidth {
    width: 100%;

    .textarea {
      width: 100%;
    }
  }

  .textarea-maxlength {
    margin-top: var(--padding-xs);
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    align-self: flex-end;

    &.error {
      color: var(--color-error);
    }
  }
}
</style>
