import type { InputValidationRule } from '$lib/utils/input'
import type { FormElementProps, InputType } from '../input'

export type InputProps = {
  type?: InputType
  value?: string
  placeholder?: string
  fullwidth?: boolean
  live?: boolean
  validation?: InputValidationRule
} & FormElementProps
