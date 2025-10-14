import type { Snippet } from 'svelte'
import type { Color } from '../color'

export type ChipProps = {
  children: Snippet
  color?: Color
  solid?: boolean
  outline?: boolean
}
