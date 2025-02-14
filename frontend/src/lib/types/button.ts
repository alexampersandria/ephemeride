import type { PrimaryColor } from './color'
import type { Icon } from './icon'

export type ButtonVariant = 'primary' | 'secondary' | 'invisible'

export type ButtonProps = {
  variant?: ButtonVariant
  color?: PrimaryColor
  loading?: boolean
  disabled?: boolean
  icon?: Icon
}
