import type { InputFieldAttributes, InputType, ValidationRule } from '../input'

export type InputProps = {
  type?: InputType
  value?: string
  placeholder?: string
  fullwidth?: boolean
  live?: boolean
  validation?: ValidationRule
} & InputFieldAttributes
