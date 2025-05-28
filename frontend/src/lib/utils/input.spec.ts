import { expect, it, describe } from 'vitest'
import { evaluateInputState } from './input'

describe('evaluateInputState', () => {
  it('should return "untouched" if value is empty and state is untouched', () => {
    const result = evaluateInputState({
      value: '',
      inputstate: 'untouched',
    })
    expect(result).toBe('untouched')
  })

  it('should return "touched" if value is empty and state is touched', () => {
    const result = evaluateInputState({
      value: '',
      inputstate: 'touched',
    })
    expect(result).toBe('touched')
  })

  it('should return "touched" if value is not empty', () => {
    const result = evaluateInputState({
      value: 'test',
      inputstate: 'untouched',
    })
    expect(result).toBe('touched')
  })

  it('should return "invalid" if value is empty and required is true', () => {
    const result = evaluateInputState({
      value: '',
      inputstate: 'untouched',
      required: true,
    })
    expect(result).toBe('invalid')
  })
})

describe('evaluateInputState with regex', () => {
  it('should return "untouched" if value is empty and state is untouched', () => {
    const result = evaluateInputState({
      value: '',
      inputstate: 'untouched',
      validation: /test/,
    })
    expect(result).toBe('untouched')
  })

  it('should return "touched" if value is empty and state is touched', () => {
    const result = evaluateInputState({
      value: '',
      inputstate: 'touched',
      validation: /test/,
    })
    expect(result).toBe('touched')
  })

  it('should return "touched" if value matches regex', () => {
    const result = evaluateInputState({
      value: 'test',
      inputstate: 'untouched',
      validation: /test/,
    })
    expect(result).toBe('touched')
  })

  it('should return "invalid" if value does not match regex', () => {
    const result = evaluateInputState({
      value: 'test',
      inputstate: 'untouched',
      validation: /invalid/,
    })
    expect(result).toBe('invalid')
  })
})

describe('evaluateInputState with function', () => {
  it('should return the expected result', () => {
    const validation = (value: string) => {
      return value === 'test' ? 'touched' : 'invalid'
    }

    const resultInvalid = evaluateInputState({
      value: '',
      inputstate: 'untouched',
      validation,
    })
    const resultValid = evaluateInputState({
      value: 'test',
      inputstate: 'untouched',
      validation,
    })

    expect(resultInvalid).toBe('invalid')
    expect(resultValid).toBe('touched')
  })
})
