<script lang="ts">
import type { MoodInputProps, MoodValue } from '$lib/types/components/moodinput'

let { value = $bindable() }: MoodInputProps = $props()

const setValue = (mood: number) => () => {
  value = mood as MoodValue
}
</script>

<div class="mood-input">
  {#each [1, 2, 3, 4, 5] as mood}
    <button
      type="button"
      class="mood-button mood-{mood}"
      class:selected={value === mood}
      onclick={setValue(mood)}
      aria-label={`Select mood ${mood}`}>
      {mood}
    </button>
  {/each}
</div>

<style lang="scss">
.mood-input {
  display: flex;
  gap: var(--mood-button-gap);
  justify-content: center;

  .mood-button {
    width: var(--mood-button-size);
    height: var(--mood-button-size);
    border: none;
    border-radius: 50%;
    transition: box-shadow 0.1s ease-out;

    @each $mood in 1, 2, 3, 4, 5 {
      &.mood-#{$mood} {
        background-color: var(--mood-background-#{$mood});
        color: var(--mood-color-#{$mood});

        &:hover {
          background-color: var(--mood-background-hover-#{$mood});
        }

        &:active {
          background-color: var(--mood-background-active-#{$mood});
        }

        &.selected {
          background-color: var(--mood-background-selected-#{$mood});
          color: var(--background-primary);
          box-shadow:
            0 0 0 2px var(--background-primary),
            0 0 0 4px var(--mood-background-selected-#{$mood});
        }
      }
    }

    &:active {
      transform: var(--click-transform);
    }
  }
}
</style>
