<script lang="ts">
import { ChartLine } from 'lucide-svelte'
</script>

<div class="stats-page">
  <div class="container">
    <div class="title">
      <ChartLine />
      Stats (WIP)
    </div>
  </div>
</div>

<style lang="scss">
.stats-page {
  padding: var(--padding-l);

  .title {
    font-size: var(--font-size-l);
    display: flex;
    align-items: center;
    gap: var(--padding-s);
    margin-bottom: var(--padding-m);
  }
}
</style>
