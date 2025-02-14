<script lang="ts">
import type { BannerProps } from '$lib/types/banner'

let { children, message, actions, variant = 'info' }: BannerProps = $props()
</script>

<div class="alert {variant}">
  <div class="icon">
    <div class="todo"></div>
  </div>
  <div class="message">
    {#if children}
      {@render children()}
    {:else if message}
      {@render message()}
    {/if}
  </div>
  {#if actions}
    <div class="actions">
      {@render actions()}
    </div>
  {/if}
</div>

<style lang="scss">
.alert {
  flex: 0 0 auto;
  display: flex;
  align-items: center;
  width: 100%;
  padding: var(--padding-m);
  border-radius: var(--radius-m);
  border: var(--border-width) solid var(--border-color);

  --color-link: var(--text-muted);
  --color-link-hover: var(--text-primary);

  &.error {
    background-color: var(--color-error-bg);
    color: var(--color-error);
    border-color: var(--color-error);
  }

  &.warning {
    background-color: var(--color-warning-bg);
    color: var(--color-warning);
    border-color: var(--color-warning);
  }

  &.success {
    background-color: var(--color-success-bg);
    color: var(--color-success);
    border-color: var(--color-success);
  }

  &.info {
    background-color: var(--color-info-bg);
    color: var(--color-info);
    border-color: var(--color-info);
  }

  .actions {
    display: flex;
    gap: var(--padding-m);
    margin-left: auto;
  }
}
</style>
