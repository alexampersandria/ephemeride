import type { InputState } from "../input"

export type AuthModelField = {
  value: string,
  inputstate: InputState
}

export type AuthModel = {
  name: AuthModelField,
  email: AuthModelField,
  password: AuthModelField,
  inviteCode: AuthModelField
}