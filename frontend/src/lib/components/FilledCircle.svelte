<script lang="ts">
import type { FilledCircleProps } from '$lib/types/components/filledcircle'

let { percentage = 0, color = 'base' }: FilledCircleProps = $props()
</script>

<div
  class="filled-circle {color}"
  style="--percentage: {Math.min(
    percentage,
    100,
  )}; --circle-fill-color: var(--color-{color}-60);">
  <div class="inner"></div>
</div>

<style lang="scss">
.filled-circle {
  width: var(--filled-circle-size);
  height: var(--filled-circle-size);
  border-radius: 50%;

  --circle-background-color: var(--background-accent);
  --circle-fill-color: var(--color-blue-60);

  background: conic-gradient(
    var(--circle-fill-color) calc(var(--percentage) * 1%),
    var(--circle-background-color) calc(var(--percentage) * 1%)
  );

  .inner {
    width: calc(100% - 4px);
    height: calc(100% - 4px);
    background-color: var(--background-primary);
    border-radius: 50%;
    position: relative;
    top: 2px;
    left: 2px;
  }
}
</style>
