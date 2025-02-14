import type { Snippet } from 'svelte'
import type { NotificationState } from './notification'

export type BannerProps = {
  children?: Snippet
  message?: Snippet
  actions?: Snippet
  variant?: NotificationState
}
