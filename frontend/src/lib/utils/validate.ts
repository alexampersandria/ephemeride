import type { InputState } from '$lib/types/input'

export type ValidationFunction = (
  value: string,
  state?: InputState,
) => InputState
export type InputValidationRule = ValidationFunction | RegExp

export const validateInput = (args: {
  value: string
  state: InputState
  validation?: InputValidationRule
  required?: boolean
}): InputState => {
  console.log('args', args)

  if (args.validation) {
    if (typeof args.validation === 'function') {
      return args.validation(args.value, args.state)
    } else {
      if (!args.value) {
        if (args.required) {
          return 'invalid'
        } else {
          if (args.state === 'untouched') {
            return 'untouched'
          } else {
            return 'touched'
          }
        }
      } else {
        return args.validation.test(args.value) ? 'touched' : 'invalid'
      }
    }
  } else {
    console.log(args)
    if (!args.value) {
      if (args.required) {
        return 'invalid'
      } else {
        return args.state
      }
    } else {
      return 'touched'
    }
  }
}
