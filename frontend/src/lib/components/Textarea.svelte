<script lang="ts">
import type { TextareaProps } from '$lib/types/components/textarea'
import { onMount } from 'svelte'
import FilledCircle from './FilledCircle.svelte'
import { formatNumber } from '$lib/utils/numbers'

let {
  value = $bindable(''),
  placeholder,
  disabled,
  name,
  id,
  required,
  maxlength,
  fullwidth,
  inputstate = $bindable('untouched'),
  onchange: emitOnChange,
  oninput: emitOnInput,
}: TextareaProps = $props()

let exceedsMaxLength = $derived.by(() => {
  if (value && maxlength) {
    return value.length > maxlength
  } else {
    return false
  }
})

const oninput = (event: Event) => {
  evaluateInputState()

  if (emitOnInput) {
    emitOnInput(event)
  }
}

const onchange = (event: Event) => {
  if (emitOnChange) {
    emitOnChange(event)
  }
}

const evaluateInputState = (mount = false) => {
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
  } else if (!mount) {
    if (required) {
      inputstate = 'invalid'
    } else {
      inputstate = 'touched'
    }
  }
}

onMount(() => {
  console.log(value)
  evaluateInputState(true)
})

const circlePercentage = $derived.by(() => {
  if (maxlength) {
    if (value) {
      return Math.min((value.length / maxlength) * 100, 100)
    } else {
      return 0
    }
  } else {
    return 0
  }
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
    {oninput}
    {onchange}>
  </textarea>

  {#if maxlength}
    <div
      class="textarea-maxlength"
      class:error={exceedsMaxLength}
      aria-live="polite"
      aria-atomic="true">
      <div class="length-text">
        {formatNumber(value.length)} / {formatNumber(maxlength)}
      </div>
      <div class="circle">
        <FilledCircle
          percentage={circlePercentage}
          color={exceedsMaxLength ? 'red' : 'base'} />
      </div>
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
    height: calc(6lh + var(--padding-s) * 2);
    min-height: calc(2lh + var(--padding-s) * 2);

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
    display: flex;
    align-items: center;
    gap: var(--padding-xs);
    margin-top: var(--padding-xs);
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    align-self: flex-end;

    &.error {
      color: var(--color-error);
    }

    .circle {
      position: relative;
      top: -1px;
    }
  }
}
</style>
