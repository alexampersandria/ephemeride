export type InputState = 'untouched' | 'touched' | 'invalid'
export type InputType = 'text' | 'password' | 'email' | 'number' | 'tel' | 'url'

export type ButtonType = 'primary' | 'secondary' | 'ghost' | 'destructive'

export type InputFieldAttributes = {
  disabled?: boolean
  state?: InputState
  name?: string
  required?: boolean
  id?: string
}

export type ValidationFunction = (
  value: string,
  state?: InputState,
) => InputState

export type ValidationRule = ValidationFunction | RegExp
