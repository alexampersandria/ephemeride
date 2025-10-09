<script lang="ts">
import type { FilledCircleProps } from '$lib/types/components/filledcircle'

let { percentage = 0, color = 'base' }: FilledCircleProps = $props()
</script>

<div
  class="filled-circle {color}"
  style="--percentage: {Math.min(percentage, 100)};">
  <div class="inner"></div>
</div>

<style lang="scss">
@use '../assets/scss/color';

.filled-circle {
  width: var(--filled-circle-size);
  height: var(--filled-circle-size);
  border-radius: 50%;

  --circle-fill-color: var(--circle-fill-color-base);

  background: conic-gradient(
    var(--circle-fill-color) calc(var(--percentage) * 1%),
    var(--circle-background-color) calc(var(--percentage) * 1%)
  );

  @each $color in color.$colors {
    &.#{$color} {
      --circle-fill-color: var(--circle-fill-color-#{$color});
    }
  }

  .inner {
    width: calc(100% - var(--filled-circle-thickness) * 2);
    height: calc(100% - var(--filled-circle-thickness) * 2);
    background-color: var(--background-primary);
    border-radius: 50%;
    position: relative;
    top: var(--filled-circle-thickness);
    left: var(--filled-circle-thickness);
  }
}
</style>
