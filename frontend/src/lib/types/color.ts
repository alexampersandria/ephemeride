export const saturatedColors = [
  'base',
  'blue',
  'green',
  'red',
  'yellow',
  'purple',
  'pink',
] as const

export type SaturatedColor = (typeof saturatedColors)[number]

export const colors = [...saturatedColors, 'base'] as const

export type Color = (typeof colors)[number]
