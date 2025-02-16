import type { Snippet } from 'svelte'
import type { MessageType } from '../message'

export type MessageProps = {
  children: Snippet
  type?: MessageType
  colortext?: boolean
  size?: 'small' | 'medium' | 'large'
}
