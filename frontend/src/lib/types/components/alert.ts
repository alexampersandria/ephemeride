import type { Snippet } from 'svelte'
import type { MessageProps } from './message'

export type AlertProps = {
  message?: Snippet
  actions?: Snippet
  solid?: boolean
} & MessageProps
