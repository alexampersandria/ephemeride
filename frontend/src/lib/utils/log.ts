import { colorPriority, moodColors } from '$lib/types/color'
import type { MoodValue } from '$lib/types/components/moodinput'
import type { CategoryWithTags } from '$lib/types/log'

/**
 * Sort categories alphabetically by name, ignoring case.
 * And sorts tags within each category, first by color, then by name.
 * @see /src/lib/types/color.ts colorPriority
 * @param categories - categories to sort
 */
export const sortCategories = (categories: CategoryWithTags[]) => {
  const copy = JSON.parse(JSON.stringify(categories)) as CategoryWithTags[]
  const sortedCategories = copy.sort((a, b) =>
    a.name.localeCompare(b.name, undefined, { sensitivity: 'base' }),
  )

  sortedCategories.forEach(category => {
    category.tags.sort((a, b) => {
      if (a.color === b.color) {
        return a.name.localeCompare(b.name, undefined, { sensitivity: 'base' })
      }
      return colorPriority[b.color] - colorPriority[a.color]
    })
  })

  return sortedCategories
}

export const moodColor = (mood: MoodValue) => {
  return moodColors[mood]
}

/**
 * Formats a date string (YYYY-MM-DD) to a more readable format, used for detailed views
 * @param dateString - YYYY-MM-DD
 * @returns formatted date string like "Monday, January 1, 2023"
 */
export const fullDate = (dateString: string) => {
  const date = new Date(dateString)

  const weekday = date.toLocaleDateString(undefined, { weekday: 'long' })
  const month = date.toLocaleDateString(undefined, { month: 'long' })
  const day = date.getDate()
  const year = date.getFullYear()

  return `${weekday}, ${month} ${day}, ${year}`
}
