<script lang="ts">
import type { ChipProps } from '$lib/types/components/chip'
import { onMount } from 'svelte'

let {
  children,
  color = 'base',
  variant = 'subtle',
  outline,
}: ChipProps = $props()

let chip: HTMLDivElement | null = null
let isSingleCharacter = $state(false)

const evalSingleCharacter = () => {
  if (!chip) return
  const content = chip.textContent?.trim() || ''
  const hasOnlyIcon =
    content.length === 0 &&
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
  class:outline
  class:single-character={isSingleCharacter}
  bind:this={chip}>
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
  box-shadow:
    0 0 0 0 var(--background-primary),
    0 0 0 0 var(--chip-background-color);
  transition: box-shadow 0.05s ease-out;
  min-height: 1.899rem;

  :global(.lucide) {
    min-width: 1em;
    min-height: 1em;
  }

  &.single-character {
    // #TODO: maybe figure out a better way to do this, like fixed height instead of padding and then we can set the width to the same or use aspect ratio?
    width: 1.899rem;
  }

  &.outline {
    box-shadow:
      0 0 0 var(--focus-shadow-offset) var(--background-primary),
      0 0 0 calc(var(--focus-shadow-offset) + var(--focus-shadow-offset))
        var(--chip-background-color);
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
  border-radius: 9999px;

  &:hover .chip {
    background-color: var(--chip-background-color-hover);
  }

  &:active .chip {
    background-color: var(--chip-background-color-active);
    transform: var(--click-transform);
  }
}
</style>
