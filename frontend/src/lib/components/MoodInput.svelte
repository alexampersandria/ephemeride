<script lang="ts">
import type { MoodInputProps, MoodValue } from '$lib/types/components/moodinput'

let {
  mode = 'edit',
  value = $bindable(),
  'aria-label': ariaLabel,
  inputstate = $bindable('untouched'),
}: MoodInputProps = $props()

const setValue = (mood: number) => () => {
  if (mode === 'edit') {
    value = mood as MoodValue
    inputstate = 'touched'
  }
}
</script>

<div
  class="mood-input mood-value-{value || 'none'} mode-{mode}"
  aria-label={ariaLabel}
  aria-invalid={inputstate === 'invalid'}>
  <div class="mood-buttons-container">
    {#each [1, 2, 3, 4, 5] as mood}
      <button
        class="plain mood-button mood-{mood}"
        class:selected={value === mood}
        onclick={setValue(mood)}
        disabled={mode === 'view'}
        aria-label={mode === 'edit' ? `Select mood ${mood}` : `Mood ${mood}`}
        aria-disabled={mode === 'view'}>
        <div class="mood-button-inner">
          {mood}
        </div>
      </button>
    {/each}
  </div>
  <div class="mood-bar-wrapper">
    {#each ['filled', 'base'] as variant}
      <div class="mood-bar {variant} mood-value-{value || 'none'}">
        {#each [1, 2, 3, 4, 5] as _}
          <div class="mood-bar-segment">
            {#each [1, 2, 3, 4, 5] as _}
              <div class="mood-bar-segment-bar"></div>
            {/each}
          </div>
        {/each}
      </div>
    {/each}
  </div>
</div>

<style lang="scss">
.mood-input {
  display: flex;
  justify-content: center;
  flex-direction: column;
  width: 100%;
  max-width: var(--block-size-s);

  --mood-color-chosen-primary: var(--color-base-60);
  --mood-color-chosen-secondary: var(--color-base-70);
  --mood-color-chosen-tertiary: var(--color-base-90);

  @each $mood in (1, 2, 3, 4, 5) {
    &.mood-value-#{$mood} {
      --mood-color-chosen-primary: var(--mood-button-selected-primary-#{$mood});
      --mood-color-chosen-secondary: var(
        --mood-button-selected-secondary-#{$mood}
      );
      --mood-color-chosen-tertiary: var(
        --mood-button-selected-tertiary-#{$mood}
      );
    }
  }

  .mood-buttons-container {
    display: flex;
    justify-content: space-around;
    width: 100%;

    .mood-button {
      height: calc(var(--mood-button-height) + var(--mood-bar-height-primary));
      flex: 1 1 0;
      transition: color var(--animation-length-s) var(--better-ease-out);

      .mood-button-inner {
        height: var(--mood-button-height);
      }

      &.selected {
        color: var(--mood-color-chosen-primary);
      }

      &:disabled {
        cursor: default;
      }
    }

    &:has(.selected) {
      .mood-button:not(.selected) {
        color: var(--mood-button-unselected);
      }
    }
  }

  &.mode-edit {
    .mood-buttons-container:has(.selected) {
      .mood-button {
        &:not(.selected):hover {
          color: var(--text-primary);
        }

        &:active {
          .mood-button-inner {
            transform: var(--click-transform);
          }
        }
      }
    }
  }

  .mood-bar-wrapper {
    position: relative;
    top: calc(var(--mood-bar-height-primary) * -1);
    pointer-events: none;
    width: 100%;

    .mood-bar {
      position: absolute;
      top: 0;
      width: 100%;
      display: flex;
      justify-content: space-between;

      .mood-bar-segment {
        display: flex;
        justify-content: space-between;
        align-items: flex-end;
        width: 100%;

        .mood-bar-segment-bar {
          width: var(--mood-bar-width);
          height: var(--mood-bar-height-primary);
          background-color: var(--mood-bar-color-primary);
          border-radius: var(--mood-bar-width);

          &:not(:first-child, :last-child) {
            height: var(--mood-bar-height-secondary);
            background-color: var(--mood-bar-color-secondary);
          }
        }

        &:not(:last-child) {
          .mood-bar-segment-bar {
            &:last-child {
              width: 0;
            }
          }
        }
      }

      &.filled {
        z-index: 1;
        clip-path: rect(0 0 100% 0);
        transition: clip-path var(--animation-length-m) var(--better-ease-out);

        &:after {
          content: '';
          position: absolute;
          bottom: 0;
          left: 0;
          width: 100%;
          height: var(--mood-bar-width);
          background-color: var(--mood-color-chosen-tertiary);
          border-radius: var(--mood-bar-width);
          transition: background-color var(--animation-length-l) var(--better-ease-out);
          z-index: -1;
        }

        @each $mood in (1, 2, 3, 4) {
          &.mood-value-#{$mood} {
            clip-path: rect(0 calc(#{$mood} * 20% - 0.25px) 100% 0);
          }
        }

        &.mood-value-5 {
          clip-path: rect(0 100% 100% 0);
        }

        .mood-bar-segment {
          .mood-bar-segment-bar {
            background-color: var(--mood-color-chosen-primary);
            transition: background-color var(--animation-length-l) var(--better-ease-out);

            &:not(:first-child, :last-child) {
              background-color: var(--mood-color-chosen-secondary);
            }
          }
        }
      }
    }
  }
}
</style>
