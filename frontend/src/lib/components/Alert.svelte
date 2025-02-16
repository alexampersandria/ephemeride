<script lang="ts">
import type { AlertProps } from '$lib/types/components/alert'
import Message from './Message.svelte'

let { children, message, actions, type = 'info' }: AlertProps = $props()
</script>

<div class="alert {type}">
  <div class="alert-message">
    <Message {type}>
      {#if children}
        {@render children()}
      {:else if message}
        {@render message()}
      {/if}
    </Message>
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

  &:has(.actions) {
    .alert-message {
      padding: 0 var(--padding-xs);
    }
  }

  .actions {
    display: flex;
    gap: var(--alert-padding);
    margin-left: auto;
  }
}
</style>
