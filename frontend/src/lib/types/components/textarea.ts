import type { InputFieldAttributes } from '../input'

export type TextareaProps = {
  value?: string
  maxlength?: number
  placeholder?: string
  fullwidth?: boolean
} & InputFieldAttributes
