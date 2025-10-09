<script lang="ts">
import type { ChipProps } from '$lib/types/components/chip'
import { onMount } from 'svelte'

let { children, color = 'base', variant = 'subtle' }: ChipProps = $props()

let chip: HTMLDivElement | null = null
let isSingleCharacter = $state(false)

const evalSingleCharacter = () => {
  if (!chip) return
  const content = chip.textContent?.trim() || ''
  const hasOnlyIcon =
    chip.children.length === 1 &&
    chip.children[0].tagName === 'svg' &&
    chip.children[0].classList.contains('lucide')
  isSingleCharacter = content.length === 1 || hasOnlyIcon
}

onMount(() => {
  evalSingleCharacter()

  const observer = new MutationObserver(() => {
    evalSingleCharacter()
  })

  observer.observe(chip!, {
    childList: true,
    subtree: true,
    characterData: true,
  })
})
</script>

<div
  class="chip {color} {variant}"
  bind:this={chip}
  class:single-character={isSingleCharacter}>
  {@render children()}
</div>

<style lang="scss">
@use '../assets/scss/color';

.chip {
  vertical-align: middle;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 9999px;
  font-size: var(--font-size-xs);
  padding: var(--padding-xxs) var(--padding-s);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;

  :global(.lucide) {
    min-width: 1em;
    min-height: 1em;
  }

  &.single-character {
    max-width: 1.765625rem;
    aspect-ratio: 1 / 1;
  }

  @each $color in color.$colors {
    &.#{$color} {
      @each $variant in ('subtle', 'solid') {
        &.#{$variant} {
          --chip-background-color: var(--chip-#{$variant}-#{$color}-background);
          --chip-background-color-hover: var(
            --chip-#{$variant}-#{$color}-background-hover
          );
          --chip-background-color-active: var(
            --chip-#{$variant}-#{$color}-background-active
          );
          --chip-text-color: var(--chip-#{$variant}-#{$color}-color);
        }
      }
    }
  }

  background-color: var(--chip-background-color);
  color: var(--chip-text-color);
}

:global(button:has(.chip)) {
  border: none;
  background: none;
  padding: 0;
  margin: 0;
  font: inherit;
  color: inherit;
  cursor: pointer;
}

:global(a:has(.chip)),
:global(button:has(.chip)) {
  &:hover .chip {
    background-color: var(--chip-background-color-hover);
  }

  &:active .chip {
    background-color: var(--chip-background-color-active);
    transform: var(--click-transform);
  }
}
</style>
