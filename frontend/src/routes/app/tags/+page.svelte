<script lang="ts">
import { Tags } from 'lucide-svelte'
</script>

<div class="tags-page">
  <div class="container">
    <div class="title">
      <Tags />
      Tags (WIP)
    </div>
  </div>
</div>

<style lang="scss">
.tags-page {
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
