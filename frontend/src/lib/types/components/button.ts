import type { Snippet } from 'svelte'
import type { ButtonType } from '../input'

export type ButtonProps = {
  children: Snippet
  type?: ButtonType
  loading?: boolean
  disabled?: boolean
  fullwidth?: boolean
  onclick?: () => void
  href?: string
  'aria-label'?: string
}
