import { colorPriority } from '$lib/types/color'
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
