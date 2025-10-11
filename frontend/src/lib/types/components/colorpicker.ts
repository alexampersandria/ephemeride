import type { Color } from '../color'
import type { InputFieldAttributes } from '../input'

export type ColorPickerProps = {
  colors?: Color[]
  value?: string
  onChange?: (color: string) => void
  fullwidth?: boolean
} & InputFieldAttributes
