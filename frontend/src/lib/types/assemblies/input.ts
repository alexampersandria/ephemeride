import type { InputProps } from '../components/input'

export type InputAssemblyProps = Omit<
  InputProps,
  'type' | 'placeholder' | 'validation'
>

export type PasswordInputProps = InputAssemblyProps
export type EmailInputProps = InputAssemblyProps
