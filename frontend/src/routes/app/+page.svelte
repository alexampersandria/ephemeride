<script lang="ts">
import PastSevenDays from '$lib/assemblies/PastSevenDays.svelte'
import Today from '$lib/assemblies/Today.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { currentDate, fullDate } from '$lib/utils/log'

const userStore = useUserStore()

const today = currentDate()
</script>

<div class="home-container">
  <div class="app-page">
    <div class="home-page">
      {#if userStore.userDetails}
        <div class="welcome-back">
          <div class="container fill">
            <h1>
              Welcome back
              {userStore.userDetails.name}
            </h1>
            <div class="today">
              Today is
              {fullDate(today)}
            </div>
          </div>
        </div>
      {/if}
      <div class=" info">
        <div class="container">
          <div class="home-elements">
            <Today />
            <PastSevenDays />
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<style lang="scss">
.home-container {
  min-height: 100%;
  background: linear-gradient(
    195deg,
    color-mix(var(--background-accent), transparent 33%),
    transparent
  );

  .home-page {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: var(--padding-xl);
    padding-block-start: var(--padding-l);

    .info {
      margin: 0;
      padding: 0;
    }

    .welcome-back {
      display: flex;
      flex-direction: column;
      justify-content: center;

      .today {
        color: color-mix(var(--text-primary), transparent);
      }

      h1 {
        margin: 0;
      }
    }

    .home-elements {
      display: flex;
      flex-direction: column;
      gap: var(--padding-l);
    }
  }
}
</style>
