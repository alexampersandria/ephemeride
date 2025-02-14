<script lang="ts">
import { active } from '$lib/actions/active.svelte'
import Select from '$lib/components/Select.svelte'
import { themes, useUiStore } from '$lib/store/uiStore.svelte'
import { getRoutes, routeTail } from '$lib/utils/routes.svelte'

let uiStore = useUiStore()

const design = getRoutes(/\/docs\/design\/[^\/]+\//)
const components = getRoutes(/\/docs\/components\/[^\/]+\//)
const types = getRoutes(/\/docs\/types\/[^\/]+\//)

let { children } = $props()
let themeObjects = themes.map(theme => ({
  value: theme,
  label: `${theme.charAt(0).toUpperCase()}${theme.slice(1).toLowerCase()}`,
}))
</script>

<div class="docs">
  <div class="docs-menu">
    <div class="docs-home">
      <div class="docs-route-link">
        <a href="/docs">📆 Ephemeride Documentation</a>
      </div>
    </div>
    <div class="docs-theme-changer">
      <Select options={themeObjects} bind:value={uiStore.theme} />
    </div>
  </div>

  <div class="docs-navigation">
    <div class="docs-routes">
      <div class="docs-route-title">Design</div>
      {#each design as route}
        <div class="docs-route-link">
          <a href={route} use:active>
            {routeTail(route)}
          </a>
        </div>
      {/each}
    </div>
    <div class="docs-routes">
      <div class="docs-route-title">Components</div>
      {#each components as route}
        <div class="docs-route-link">
          <a href={route} use:active>
            {routeTail(route)}
          </a>
        </div>
      {/each}
    </div>
    <div class="docs-routes">
      <div class="docs-route-title">Types</div>
      {#each types as route}
        <div class="docs-route-link">
          <a href={route} use:active>
            {routeTail(route)}
          </a>
        </div>
      {/each}
    </div>
  </div>
  <div class="docs-content">
    <div class="container">
      {@render children()}
    </div>
  </div>
</div>

<style lang="scss">
.docs {
  display: grid;
  grid-template-columns: 16rem 1fr;
  grid-template-rows: 6rem 1fr;
  grid-column-gap: var(--border-width);
  grid-row-gap: var(--border-width);
  height: 100vh;
  overflow: hidden;

  background-color: var(--border-color);
  min-height: 100vh;

  .docs-menu {
    grid-area: 1 / 1 / 2 / 3;
  }

  .docs-navigation {
    grid-area: 2 / 1 / 3 / 2;
  }

  .docs-content {
    grid-area: 2 / 2 / 3 / 3;
  }

  .docs-menu,
  .docs-content {
    padding: var(--padding-m);
  }

  .docs-menu {
    display: flex;
    align-items: center;
    background-color: var(--background-secondary);

    .docs-home {
      a {
        color: var(--text-primary);
      }
    }

    .docs-theme-changer {
      margin-left: auto;
    }
  }

  .docs-navigation,
  .docs-content {
    background-color: var(--background-primary);
    height: 100%;
    overflow: auto;
    padding-bottom: 12vh;
  }

  .docs-navigation {
    .docs-routes {
      display: flex;
      flex-direction: column;
      padding-top: var(--padding-m);

      .docs-route-link,
      .docs-route-title {
        flex: 1;
      }

      .docs-route-link a,
      .docs-route-title {
        display: block;
        padding: var(--padding-s) var(--padding-m);
      }

      .docs-route-title {
        font-weight: 700;
      }
    }
  }

  a {
    color: var(--text-dimmed);

    &:hover {
      background-color: var(--background-secondary);
      color: var(--text-muted);
    }

    &:global(.active) {
      color: var(--text-primary);
    }
  }
}
</style>
