<script lang="ts">
import type { MessageProps } from '$lib/types/components/message'
import {
  Bell,
  CircleAlert,
  CircleCheck,
  InfoIcon,
  TriangleAlert,
} from 'lucide-svelte'

let { children, type = 'info' }: MessageProps = $props()

let MessageIcon = $derived.by(() => {
  switch (type) {
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

<div class="message {type}">
  <div class="message-icon lucide-icon-alignment">
    <MessageIcon />
  </div>
  <div class="message-content">
    {@render children()}
  </div>
</div>

<style lang="scss">
.message {
  display: flex;
  align-items: flex-start;
  gap: var(--message-gap);

  &.error {
    .message-icon {
      color: var(--color-error);
    }
  }

  &.warning {
    .message-icon {
      color: var(--color-warning);
    }
  }

  &.success {
    .message-icon {
      color: var(--color-success);
    }
  }

  &.info {
    .message-icon {
      color: var(--color-info);
    }
  }
}
</style>
