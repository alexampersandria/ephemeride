import type { Snippet } from 'svelte'
import type { Color } from './color'
import type { Icon } from './icon'

export type ButtonVariant = 'primary' | 'secondary' | 'invisible'

export type ButtonProps = {
  children: Snippet
  variant?: ButtonVariant
  color?: Color
  loading?: boolean
  disabled?: boolean
  icon?: Icon
}
