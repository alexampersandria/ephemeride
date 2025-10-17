<script lang="ts">
import { page } from '$app/state'
import { active } from '$lib/actions/active.svelte'
import { useUiStore } from '$lib/store/uiStore.svelte'
import {
  getRoutes,
  componentNameFromRoute,
  titleFromRoute,
} from '$lib/utils/routes.svelte'
import Logo from '$lib/components/Logo.svelte'
import { Github, Menu, Moon, Sun, X } from 'lucide-svelte'

let uiStore = useUiStore()

const design = getRoutes(/\/docs\/design\/[^/]+\//)
const components = getRoutes(/\/docs\/components\/[^/]+\//)
const assemblies = getRoutes(/\/docs\/assemblies\/[^/]+\//)
const types = getRoutes(/\/docs\/types\/[^/]+\//)

let { children } = $props()

let sidebarOpen = $state(false)
let content: HTMLElement

$effect(() => {
  if (page.route.id) {
    // close sidebar and scroll to top when navigating to a new page
    sidebarOpen = false
    content.scrollTop = 0
  }
})

const sidebarToggle = () => {
  sidebarOpen = !sidebarOpen
}

const SidebarIcon = $derived.by(() => {
  if (sidebarOpen) {
    return X
  } else {
    return Menu
  }
})

const themeToggle = () => {
  if (uiStore.appliedTheme === 'dark') {
    uiStore.theme = 'light'
  } else {
    uiStore.theme = 'dark'
  }
}

const ThemeToggleIcon = $derived.by(() => {
  if (uiStore.appliedTheme === 'dark') {
    return Sun
  } else {
    return Moon
  }
})

const title = $derived(
  [...page.url.pathname.split('/').slice(2)]
    .filter(Boolean)
    .map(part =>
      part.replace(/-|_/g, ' ').replace(/\b\w/g, l => l.toUpperCase()),
    )
    .join('/') + ' — Ephemeride Documentation',
)
</script>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<div class="docs">
  <div class="docs-menu">
    <div class="docs-sidebar-toggle">
      <button onclick={sidebarToggle}>
        <SidebarIcon />
      </button>
    </div>
    <div class="docs-home">
      <div class="docs-route-link">
        <a href="/">
          <Logo text />
        </a>
      </div>
    </div>
    <div class="docs-icon-actions">
      <button class="theme-changer" onclick={themeToggle}>
        <ThemeToggleIcon />
      </button>
      <a
        class="github-link"
        href="https://github.com/alexampersandria/ephemeride"
        target="_blank">
        <Github />
      </a>
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
      <div class="docs-route-title">Assemblies</div>
      {#each assemblies as route}
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

  .docs-menu {
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

    .docs-icon-actions,
    .docs-sidebar-toggle {
      button,
      a {
        background-color: transparent;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        color: var(--text-muted);

        &:hover {
          color: var(--text-primary);
        }

        &:active {
          color: var(--text-muted);
        }
      }
    }

    .docs-icon-actions {
      display: flex;
      align-items: center;
      gap: var(--padding-s);
      margin-left: auto;
      padding-right: var(--padding-s);
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

  .docs-content {
    padding: var(--padding-l) 0;
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
      background-color: transparent;
      transition:
        background-color 0.15s ease-out,
        backdrop-filter 0.3s ease-out;
      pointer-events: none;
      z-index: 1311;

      &:has(+ .sidebarOpen) {
        background-color: var(--background-overlay);
        backdrop-filter: blur(4px);
      }
    }

    .docs-navigation {
      position: fixed;
      top: calc(6rem + var(--border-width));
      border-right: var(--border-width) solid var(--border-color);
      left: calc(-100% - var(--border-width));
      width: 100%;
      z-index: 1312;

      transition: left 0.3s;

      &.sidebarOpen {
        left: 0;
      }
    }

    .docs-menu {
      justify-content: space-between;

      .docs-home {
        flex: 1;
      }

      .docs-sidebar-toggle {
        display: block;
      }
    }
  }
}
</style>
