<script lang="ts">
import type { HeatmapProps } from '$lib/types/components/heatmap'
import { currentDateObject, weeksInYear } from '$lib/utils/log'

let {
  data,
  year = currentDateObject().year,
  loading = false,
}: HeatmapProps = $props()

let weeks = $derived.by(() => {
  return weeksInYear(year)
})

let moodValue = (date: string) => {
  if (data) {
    const dataPoint = data.find(point => point.date === date)
    if (dataPoint) {
      return dataPoint.value
    }
  }
  return null
}
</script>

<div class="heatmap-wrapper">
  <div class="heatmap" class:loading>
    {#each weeks as week, weekIndex}
      <div class="week week-{weekIndex}">
        {#each week as day}
          {@const value = moodValue(day)}
          <a
            href={`/app/entry/${day}`}
            class="day mood-{value ?? 'null'} "
            aria-label={`${day}: ${value ?? 'null'}`}>
            <div class="square"></div>
          </a>
        {/each}
      </div>
    {/each}
  </div>
</div>

<style lang="scss">
.heatmap-wrapper {
  display: flex;
  width: 100%;
  justify-content: center;
  background-color: var(--heatmap-background);
  border-radius: var(--heatmap-border-radius);
  border: var(--border-width) solid var(--border-color);

  .heatmap {
    display: flex;
    padding: var(--heatmap-padding);

    overflow-x: auto;

    .week {
      display: flex;
      flex-direction: column;

      &:first-child {
        align-self: flex-end;
      }

      @for $i from 1 through 53 {
        &.week-#{$i} {
          .day {
            &:after {
              animation-delay: calc(0.005s * #{$i});
            }
          }
        }
      }

      .day {
        padding: calc(var(--heatmap-cell-gap) / 2);
        position: relative;

        &:after {
          content: '';
          position: absolute;
          top: calc(var(--heatmap-cell-gap) / 2);
          left: calc(var(--heatmap-cell-gap) / 2);
          z-index: 2;
          display: block;
          width: var(--heatmap-cell-size);
          height: var(--heatmap-cell-size);
          border-radius: var(--heatmap-cell-border-radius);
          background-color: var(--heatmap-cell-background);

          animation: loadingAnimation var(--animation-length-xxl)
            var(--better-ease-out) infinite;

          opacity: 0;
          transition: opacity var(--animation-length-m) var(--better-ease-out);
        }

        @for $i from 1 through 5 {
          &.mood-#{$i} {
            .square {
              background-color: var(--mood-value-#{$i}-background);

              &:hover {
                background-color: var(--mood-value-#{$i}-background-hover);
                box-shadow: var(--mood-value-#{$i}-glow);
                z-index: 1;
              }

              &:active {
                background-color: var(--mood-value-#{$i}-background-active);
              }
            }
          }
        }

        .square {
          position: relative;
          width: var(--heatmap-cell-size);
          height: var(--heatmap-cell-size);
          border-radius: var(--heatmap-cell-border-radius);
          background-color: var(--heatmap-cell-background);
        }
      }
    }

    &:has(.day:not(.mood-null):hover) {
      .day {
        @for $i from 1 through 5 {
          &:not(:hover) {
            &.mood-#{$i} {
              .square {
                background-color: var(--mood-value-#{$i}-background-muted);
              }
            }
          }
        }
      }
    }

    &.loading {
      .week {
        .day {
          position: relative;

          &:after {
            opacity: 1;
          }
        }
      }

      @keyframes loadingAnimation {
        0% {
          background-color: var(--heatmap-cell-background);
        }
        50% {
          background-color: var(--heatmap-cell-background-loading);
        }
        100% {
          background-color: var(--heatmap-cell-background);
        }
      }
    }
  }
}
</style>
