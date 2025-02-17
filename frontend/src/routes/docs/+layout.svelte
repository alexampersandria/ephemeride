<script lang="ts">
import { page } from '$app/state'
import { active } from '$lib/actions/active.svelte'
import Select from '$lib/components/Select.svelte'
import { themes, useUiStore } from '$lib/store/uiStore.svelte'
import {
  getRoutes,
  componentNameFromRoute,
  titleFromRoute,
} from '$lib/utils/routes.svelte'

let uiStore = useUiStore()

const design = getRoutes(/\/docs\/design\/[^/]+\//)
const components = getRoutes(/\/docs\/components\/[^/]+\//)
const types = getRoutes(/\/docs\/types\/[^/]+\//)

let { children } = $props()
let themeObjects = themes.map(theme => ({
  value: theme,
  label: `${theme.charAt(0).toUpperCase()}${theme.slice(1).toLowerCase()}`,
}))
let sidebarOpen = $state(false)
let content: HTMLElement

$effect(() => {
  if (page.route.id) {
    // close sidebar and scroll to top when navigating to a new page
    sidebarOpen = false
    content.scrollTop = 0
  }
})
</script>

<div class="docs">
  <div class="docs-menu">
    <div class="docs-sidebar-toggle">
      <button
        onclick={() => {
          sidebarOpen = !sidebarOpen
        }}>
        ⚙️
      </button>
    </div>
    <div class="docs-home">
      <div class="docs-route-link">
        <a href="/docs">📆 Ephemeride Documentation</a>
      </div>
    </div>
    <div class="docs-theme-changer">
      <Select options={themeObjects} bind:value={uiStore.theme} />
    </div>
  </div>

  <div class="docs-navigation-backdrop"></div>
  <div class="docs-navigation" class:sidebarOpen>
    <div class="docs-routes">
      <div class="docs-route-title">Design</div>
      {#each design as route}
        <div class="docs-route-link">
          <a href={route} use:active>
            {titleFromRoute(route)}
          </a>
        </div>
      {/each}
    </div>
    <div class="docs-routes">
      <div class="docs-route-title">Components</div>
      {#each components as route}
        <div class="docs-route-link">
          <a href={route} use:active>
            {componentNameFromRoute(route)}
          </a>
        </div>
      {/each}
    </div>
    <div class="docs-routes">
      <div class="docs-route-title">Types</div>
      {#each types as route}
        <div class="docs-route-link">
          <a href={route} use:active>
            {componentNameFromRoute(route)}
          </a>
        </div>
      {/each}
    </div>
  </div>
  <div class="docs-content" bind:this={content}>
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

    .docs-sidebar-toggle {
      display: none;
    }
  }

  .docs-navigation,
  .docs-content {
    background-color: var(--background-primary);
    height: 100%;
    overflow: auto;
    padding-bottom: 25vh;
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
        padding: var(--padding-xs) var(--padding-m);
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

@media screen and (max-width: 768px) {
  .docs {
    grid-template-columns: 0 1fr;

    .docs-navigation-backdrop {
      position: fixed;
      top: calc(6rem + var(--border-width));
      left: 0;
      width: 100%;
      height: 100vh;
      background-color: var(--background-overlay);
      opacity: 0;
      transition: opacity 0.3s;
      pointer-events: none;

      &:has(+ .sidebarOpen) {
        opacity: 1;
      }
    }

    .docs-navigation {
      position: fixed;
      top: calc(6rem + var(--border-width));
      border-right: var(--border-width) solid var(--border-color);
      left: calc(-100% - var(--border-width));
      width: 100%;
      z-index: 1;

      transition: left 0.3s;

      &.sidebarOpen {
        left: 0;
      }
    }

    .docs-menu {
      .docs-sidebar-toggle {
        display: block;

        button {
          background-color: transparent;
          border: none;
          font-size: 1.5rem;
          cursor: pointer;
          margin-right: var(--padding-m);
        }
      }
    }
  }
}
</style>
