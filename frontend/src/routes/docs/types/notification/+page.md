# Notification Types

## NotificationState

The variant or type of notification. Either `error`, `warning`, `success`, or `info`. Should always default to `info`.

### Definition

```ts
export type NotificationState = 'error' | 'warning' | 'success' | 'info'
```