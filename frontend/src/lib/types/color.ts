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
