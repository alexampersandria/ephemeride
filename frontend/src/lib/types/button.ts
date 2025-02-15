import type { Snippet } from 'svelte'
import type { PrimaryColor } from './color'
import type { Icon } from './icon'

export type ButtonVariant = 'primary' | 'secondary' | 'invisible'

export type ButtonProps = {
  children: Snippet
  variant?: ButtonVariant
  color?: PrimaryColor
  loading?: boolean
  disabled?: boolean
  icon?: Icon
}
