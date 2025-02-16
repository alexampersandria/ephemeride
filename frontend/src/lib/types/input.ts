export type InputState = 'untouched' | 'touched' | 'valid' | 'invalid'
export type InputType = 'text' | 'password' | 'email' | 'number' | 'tel' | 'url'

export type ButtonVariant = 'primary' | 'secondary' | 'ghost' | 'destructive'

export type FormElementProps = {
  disabled?: boolean
  state?: InputState
  name?: string
  required?: boolean
  id?: string
}
