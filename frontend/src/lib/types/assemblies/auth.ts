import type { InputState } from '../input'

export type AuthModelField<T> = {
  value: T
  inputstate: InputState
}

export type AuthModel = {
  name: AuthModelField<string>
  email: AuthModelField<string>
  password: AuthModelField<string>
  inviteCode: AuthModelField<string>
  terms: AuthModelField<boolean>
}

export type AuthProps = {
  mode?: 'login' | 'register'
}
