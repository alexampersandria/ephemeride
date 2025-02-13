<script lang="ts">
import InternalLink from '$lib/components/InternalLink.svelte'
import { getRoutes, routeTail } from '$lib/utils/routes'

const components = getRoutes(/\/docs\/components\/[^\/]+\//)

let { children } = $props()
</script>

<div class="docs container">
  <div class="docs-menu">
    <div class="docs-home">
      <div class="docs-route-link">
        <InternalLink href="/docs">📆 Ephemeride Documentation</InternalLink>
      </div>
    </div>
  </div>

  <div class="docs-main">
    <div class="docs-navigation">
      <div class="docs-routes">
        <div class="docs-route-title">Components</div>
        {#each components as route}
          <div class="docs-route-link">
            <InternalLink href={route}>
              {routeTail(route)}
            </InternalLink>
          </div>
        {/each}
      </div>
    </div>

    <div class="docs-content">
      {@render children()}
    </div>
  </div>
</div>

<style lang="scss">
.docs {
  display: flex;
  flex-direction: column;
  gap: var(--border-width);
  background-color: var(--border-color);
  min-height: 100vh;
  border-left: var(--border-width) solid var(--border-color);
  border-right: var(--border-width) solid var(--border-color);

  .docs-menu,
  .docs-content {
    padding: var(--padding-m);
  }

  .docs-menu {
    position: sticky;
    display: flex;
    align-items: center;
    min-height: 6rem;
    flex: 0;
    background-color: var(--background-secondary);
  }

  .docs-main {
    flex: 1;
    display: flex;
    background-color: var(--border-color);
    gap: var(--border-width);

    .docs-navigation,
    .docs-content {
      background-color: var(--background-primary);
    }

    .docs-navigation {
      min-width: 14rem;

      .docs-routes {
        display: flex;
        flex-direction: column;
        padding: var(--padding-s) 0;

        .docs-route-link,
        .docs-route-title {
          flex: 1;
        }

        .docs-route-link :global(.internal-link),
        .docs-route-title {
          display: block;
          padding: var(--padding-s) var(--padding-m);
        }

        .docs-route-link {
          text-transform: capitalize;
        }

        .docs-route-title {
          font-weight: 700;
        }
      }
    }
  }

  .docs-content {
    flex: 1;
  }

  :global(.internal-link) {
    color: var(--text-dimmed);

    &:hover {
      background-color: var(--background-secondary);
      color: var(--text-muted);
    }
  }

  :global(.internal-link.active) {
    color: var(--text-primary);
  }

  .docs-home :global(.internal-link) {
    color: var(--text-primary);
  }
}
</style>
