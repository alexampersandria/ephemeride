<script lang="ts">
import type { AlertProps } from '$lib/types/alert'
import {
  Bell,
  CircleAlert,
  CircleCheck,
  InfoIcon,
  TriangleAlert,
} from 'lucide-svelte'

let { children, message, actions, variant = 'info' }: AlertProps = $props()

let AlertIcon = $derived.by(() => {
  switch (variant) {
    case 'error':
      return CircleAlert
    case 'warning':
      return TriangleAlert
    case 'success':
      return CircleCheck
    case 'info':
      return InfoIcon
    default:
      return Bell
  }
})
</script>

<div class="alert {variant}">
  <div class="message">
    <div class="icon lucide-icon-alignment"><AlertIcon /></div>
    <div class="message-content">
      {#if children}
        {@render children()}
      {:else if message}
        {@render message()}
      {/if}
    </div>
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

  padding: var(--alert-padding);
  gap: var(--alert-padding);
  border-radius: var(--radius-m);

  color: var(--alert-color);
  background-color: var(--alert-background);
  border: var(--border-width) solid var(--alert-border);

  .message {
    display: flex;
    align-items: flex-start;
    gap: var(--alert-padding);
  }

  &:has(.actions) {
    .message {
      padding: 0 var(--padding-xs);
    }
  }

  &.error {
    .icon {
      color: var(--color-error);
    }
  }

  &.warning {
    .icon {
      color: var(--color-warning);
    }
  }

  &.success {
    .icon {
      color: var(--color-success);
    }
  }

  &.info {
    .icon {
      color: var(--color-info);
    }
  }

  .actions {
    display: flex;
    gap: var(--alert-padding);
    margin-left: auto;
  }
}
</style>
