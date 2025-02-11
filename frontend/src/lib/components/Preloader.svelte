<script lang="ts">
import FullPage from './FullPage.svelte'
import Spinner from './Spinner.svelte'
</script>

<FullPage>
  <div class="preloader">
    <Spinner --spinner-size="2rem" --spinner-color="var(--base-60)" />
  </div>
</FullPage>

<style lang="scss">
.preloader {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  width: 100%;
  background-color: var(--base-00);
}
</style>
