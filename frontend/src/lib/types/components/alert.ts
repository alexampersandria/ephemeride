import type { Snippet } from 'svelte'
import type { MessageType } from '../message'

export type AlertProps = {
  children?: Snippet
  message?: Snippet
  actions?: Snippet
  type?: MessageType
}
