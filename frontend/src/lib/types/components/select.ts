import type { InputFieldAttributes } from '../input'

export type SelectOption = {
  label: string
  value: string | number | boolean
  disabled?: boolean
}

export type SelectProps = {
  options: SelectOption[]
  value?: SelectOption['value']
  placeholder?: string
  fullwidth?: boolean
} & InputFieldAttributes
