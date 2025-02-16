import type { Snippet } from 'svelte'
import type { NotificationState } from '../notification'

export type AlertProps = {
  children?: Snippet
  message?: Snippet
  actions?: Snippet
  variant?: NotificationState
}
