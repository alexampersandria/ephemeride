import type { InputState, ValidationRule } from '$lib/types/input'

export const evaluateInputState = (args: {
  value: string
  state: InputState
  validation?: ValidationRule
  required?: boolean
}): InputState => {
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
