import type { InputFieldAttributes } from '../input'

export type MoodValue = 1 | 2 | 3 | 4 | 5

export type MoodInputProps = {
  mode?: 'view' | 'edit'
  value?: MoodValue
} & InputFieldAttributes
