import { expect, it, describe } from 'vitest'
import { evaluateInputState } from './input'

describe('validateInput', () => {
  it('should return "untouched" if value is empty and state is untouched', () => {
    const result = evaluateInputState({
      value: '',
      state: 'untouched',
    })
    expect(result).toBe('untouched')
  })

  it('should return "touched" if value is empty and state is touched', () => {
    const result = evaluateInputState({
      value: '',
      state: 'touched',
    })
    expect(result).toBe('touched')
  })

  it('should return "touched" if value is not empty', () => {
    const result = evaluateInputState({
      value: 'test',
      state: 'untouched',
    })
    expect(result).toBe('touched')
  })

  it('should return "invalid" if value is empty and required is true', () => {
    const result = evaluateInputState({
      value: '',
      state: 'untouched',
      required: true,
    })
    expect(result).toBe('invalid')
  })
})

describe('validateInput with regex', () => {
  it('should return "untouched" if value is empty and state is untouched', () => {
    const result = evaluateInputState({
      value: '',
      state: 'untouched',
      validation: /test/,
    })
    expect(result).toBe('untouched')
  })

  it('should return "touched" if value is empty and state is touched', () => {
    const result = evaluateInputState({
      value: '',
      state: 'touched',
      validation: /test/,
    })
    expect(result).toBe('touched')
  })

  it('should return "touched" if value matches regex', () => {
    const result = evaluateInputState({
      value: 'test',
      state: 'untouched',
      validation: /test/,
    })
    expect(result).toBe('touched')
  })

  it('should return "invalid" if value does not match regex', () => {
    const result = evaluateInputState({
      value: 'test',
      state: 'untouched',
      validation: /invalid/,
    })
    expect(result).toBe('invalid')
  })
})

describe('validateInput with function', () => {
  it('should return the expected result', () => {
    const validation = (value: string) => {
      return value === 'test' ? 'valid' : 'invalid'
    }

    const resultInvalid = evaluateInputState({
      value: '',
      state: 'untouched',
      validation,
    })
    const resultValid = evaluateInputState({
      value: 'test',
      state: 'untouched',
      validation,
    })

    expect(resultInvalid).toBe('invalid')
    expect(resultValid).toBe('valid')
  })
})
