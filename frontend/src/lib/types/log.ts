import type { Color } from './color'
import type { MoodValue } from './components/moodinput'

export type Category = {
  id: string
  name: string
}

export type NewCategory = Omit<Category, 'id'>

export type Tag = {
  id: string
  name: string
  color: Color
}

export type NewTag = Omit<Tag, 'id'>

export type Entry = {
  id: string
  date: string
  mood: MoodValue
  entry?: string
}

export type NewEntry = Omit<Entry, 'id'>
