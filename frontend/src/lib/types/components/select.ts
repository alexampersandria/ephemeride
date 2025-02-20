import type { InputFieldAttributes } from '../input'

export type SelectOption = {
  label: string
  value: string
  disabled?: boolean
}

export type SelectProps = {
  options: SelectOption[]
  value?: string
  placeholder?: string
  fullwidth?: boolean
} & InputFieldAttributes
