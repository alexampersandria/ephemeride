export type InputState = 'untouched' | 'touched' | 'invalid'
export type InputType = 'text' | 'password' | 'email' | 'number' | 'tel' | 'url'

export type ButtonType = 'primary' | 'secondary' | 'ghost' | 'destructive'

export type InputFieldAttributes = {
  disabled?: boolean
  inputstate?: InputState
  name?: string
  required?: boolean
  id?: string
}

export type ValidationFunction = (
  value: string,
  inputstate?: InputState,
) => InputState

export type ValidationRule = ValidationFunction | RegExp
