import type { Snippet } from 'svelte'
import type { Color } from './color'
import type { ButtonVariant } from './input'

export type ButtonProps = {
  children: Snippet
  variant?: ButtonVariant
  color?: Color
  loading?: boolean
  disabled?: boolean
  onclick?: () => void
}
