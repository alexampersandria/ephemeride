import type { Snippet } from 'svelte'
import type { ButtonVariant } from './input'

export type ButtonProps = {
  children: Snippet
  variant?: ButtonVariant
  loading?: boolean
  disabled?: boolean
  onclick?: () => void
}
