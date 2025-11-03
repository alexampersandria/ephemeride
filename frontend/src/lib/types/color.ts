export const saturatedColors = [
  'blue',
  'green',
  'lime',
  'red',
  'orange',
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
  lime: 3,
  yellow: 4,
  orange: 5,
  purple: 6,
  pink: 7,
  red: 99,
}
