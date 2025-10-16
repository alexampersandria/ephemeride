import type { Snippet } from 'svelte'
import type { MessageSize } from '../message'

export type LabelProps = {
  children: Snippet
  for?: string
  weight?: 'normal' | 'bold'
  size?: MessageSize
}
