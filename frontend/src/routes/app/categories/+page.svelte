<script lang="ts">
import { FolderOpen } from 'lucide-svelte'
</script>

<div class="categories-page">
  <div class="container">
    <div class="title">
      <FolderOpen />
      Categories (WIP)
    </div>
  </div>
</div>

<style lang="scss">
.categories-page {
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
