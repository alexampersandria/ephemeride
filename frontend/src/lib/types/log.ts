import type { Color } from './color'
import type { MoodValue } from './components/moodinput'

export type Category = {
  id: string
  name: string
}

export type NewCategory = Omit<Category, 'id'>

export type CategoryWithTags = Category & {
  tags: Tag[]
  selectedTagIds?: string[]
}

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
  selectedTagIds: string[]
}

export type NewEntry = Omit<Entry, 'id'>

export const entryMaxLength = 1000

/**
 * see /backend/src/services/log.rs
 */
export const defaultCategories: CategoryWithTags[] = [
  {
    id: crypto.randomUUID(),
    name: 'Activities',
    tags: [
      { id: crypto.randomUUID(), name: 'Work', color: 'base' },
      { id: crypto.randomUUID(), name: 'Movie', color: 'base' },
      { id: crypto.randomUUID(), name: 'Exercise', color: 'base' },
      { id: crypto.randomUUID(), name: 'Read', color: 'base' },
      { id: crypto.randomUUID(), name: 'Shopping', color: 'base' },
      { id: crypto.randomUUID(), name: 'Gaming', color: 'base' },
    ],
  },
  {
    id: crypto.randomUUID(),
    name: 'Tags',
    tags: [
      { id: crypto.randomUUID(), name: 'Travel', color: 'base' },
      { id: crypto.randomUUID(), name: 'Important', color: 'blue' },
      { id: crypto.randomUUID(), name: 'Sick', color: 'red' },
    ],
  },
]
