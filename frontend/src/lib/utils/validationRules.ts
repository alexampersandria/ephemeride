import type { InputState } from '$lib/types/input'

/**
 * Generic validation rule for max length
 * @param maxLength - The maximum length of the input
 */
export const validateMaxLength = (
  value: string,
  maxLength: number,
  callback: (errors: Array<string>) => void,
): InputState => {
  const errors: Array<string> = []
  if (value.length > maxLength) {
    errors.push(`Input must be less than ${maxLength} characters`)
  }
  callback(errors)
  return errors.length === 0 ? 'touched' : 'invalid'
}

/**
 * Validation rule for email
 */
export const validateEmail = (
  value: string,
  callback: (errors: Array<string>) => void,
): InputState => {
  const errors: Array<string> = []
  if (
    !/^(([^<>()[\]\.,;:\s@\"]+(\.[^<>()[\]\.,;:\s@\"]+)*)|(\".+\"))@(([^<>()[\]\.,;:\s@\"]+\.)+[^<>()[\]\.,;:\s@\"]{2,})$/i.test(
      value,
    )
  ) {
    errors.push('Invalid email')
  }
  callback(errors)
  return errors.length === 0 ? 'touched' : 'invalid'
}

/**
 * Validation rule for password
 * Should be at least 7 characters, no more than 255 characters
 * Should include at least one uppercase letter, one lowercase letter, one number
 * Has a callback function which will pass any error messages
 */
export const validatePassword = (
  value: string,
  callback: (errors: Array<string>) => void,
): InputState => {
  const errors: Array<string> = []
  if (value.length < 7) {
    errors.push('Must be at least 7 characters')
  }
  if (value.length > 255) {
    errors.push('Must be less than 255 characters')
  }
  if (!/[A-Z]/.test(value)) {
    errors.push('Must include at least one uppercase letter')
  }
  if (!/[a-z]/.test(value)) {
    errors.push('Must include at least one lowercase letter')
  }
  if (!/[0-9]/.test(value)) {
    errors.push('Must include at least one number')
  }
  callback(errors)
  return errors.length === 0 ? 'touched' : 'invalid'
}
