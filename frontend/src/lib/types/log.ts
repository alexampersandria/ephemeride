import type { Color } from './color'
import type { MoodValue } from './components/moodinput'

export type Category = {
  id: string
  name: string
}

export type Tag = {
  id: string
  name: string
  color: Color
}

export type Entry = {
  id: string
  date: string
  mood: MoodValue
  entry?: string
}
