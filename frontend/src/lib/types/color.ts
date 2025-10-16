import type { MoodValue } from './components/moodinput'

export const saturatedColors = [
  'blue',
  'green',
  'red',
  'yellow',
  'purple',
  'pink',
] as const

export type SaturatedColor = (typeof saturatedColors)[number]

export const colors = ['base', ...saturatedColors] as const

export type Color = (typeof colors)[number]

export const colorPriority: Record<Color, number> = {
  base: 0,
  blue: 1,
  green: 2,
  yellow: 4,
  purple: 5,
  pink: 6,
  red: 99,
}

export const moodColors: Record<MoodValue, Color> = {
  1: 'red',
  2: 'yellow',
  3: 'green',
  4: 'blue',
  5: 'pink',
}
