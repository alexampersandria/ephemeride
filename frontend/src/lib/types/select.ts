import type { InputState } from './input'

export type SelectOption = {
  label: string
  value: string
  disabled?: boolean
}

export type SelectProps = {
  options: SelectOption[]
  value?: SelectOption['value']
  placeholder?: string
  disabled?: boolean
  state?: InputState
}
