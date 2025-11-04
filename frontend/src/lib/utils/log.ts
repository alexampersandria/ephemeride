import { colorPriority } from '$lib/types/color'
import type { CategoryWithTags, Entry } from '$lib/types/log'

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

/**
 * sorts entries by date first (newest first)
 */
export const sortEntries = (entries: Entry[]): Entry[] => {
  const copy = JSON.parse(JSON.stringify(entries)) as Entry[]
  const sorted = copy.sort((a, b) => {
    return new Date(b.date).getTime() - new Date(a.date).getTime()
  })
  return sorted
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

/**
 * gives the current date (YYYY-MM-DD) for the user with their local time
 * note for this definition, the current date starts at 4am, this is because quite a lot of people go to bed after midnight
 * the day start/end time will be customizable in the future
 * @returns date string YYYY-MM-DD
 */
export const currentDate = () => {
  const now = new Date()
  if (now.getHours() < 4) {
    now.setDate(now.getDate() - 1)
  }
  return now.toISOString().split('T')[0]
}

/**
 * returns currentDate but in object form
 * @returns object with year, month, day as numbers
 */
export const currentDateObject = () => {
  const date = currentDate()
  const [year, month, day] = date.split('-').map(Number)
  return { year, month, day }
}

/**
 * returns an object with the first and last date of the given month
 * defaults to current month if no parameters given
 * @param year - The year of the month
 * @param month - The month (1-12)
 * @returns object with first and last date strings YYYY-MM-DD
 */
export const monthDateRange = (year?: number, month?: number) => {
  if (!year || !month) {
    const current = currentDateObject()
    year = current.year
    month = current.month
  }

  const days: number[] = []
  calendarDaysInMonth(year, month).forEach(week => {
    week.forEach(day => {
      if (day) {
        days.push(day)
      }
    })
  })

  const firstDate = `${year}-${String(month).padStart(2, '0')}-${String(
    days[0],
  ).padStart(2, '0')}`
  const lastDate = `${year}-${String(month).padStart(2, '0')}-${String(
    days[days.length - 1],
  ).padStart(2, '0')}`

  return { firstDate, lastDate }
}

/**
 * gets the current year and month as numbers using the currentDate function
 * @returns the current year and month as numbers using the currentDate function
 */
export const calendarDefaults = (): { year: number; month: number } => {
  const date = currentDate()
  const [year, month] = date.split('-').map(Number)
  return { year, month }
}

export const formatMonth = (month: number) => {
  switch (month) {
    case 1:
      return 'January'
    case 2:
      return 'February'
    case 3:
      return 'March'
    case 4:
      return 'April'
    case 5:
      return 'May'
    case 6:
      return 'June'
    case 7:
      return 'July'
    case 8:
      return 'August'
    case 9:
      return 'September'
    case 10:
      return 'October'
    case 11:
      return 'November'
    case 12:
      return 'December'
    default:
      return ''
  }
}

/**
 * Returns an array, with the weeks of the month
 * each week is an array of either null (for days outside the month) or the day number
 * week starts on monday
 * @param year - The year of the month
 * @param month - The month (1-12)
 */
export const calendarDaysInMonth = (year: number, month: number) => {
  const daysInMonth = new Date(year, month, 0).getDate()
  const firstDayOfMonth = new Date(year, month - 1, 1).getDay()
  const weeks: (number | null)[][] = []
  let currentWeek: (number | null)[] = []

  // Fill initial nulls for days before the first of the month
  for (let i = 1; i < firstDayOfMonth; i++) {
    currentWeek.push(null)
  }

  for (let day = 1; day <= daysInMonth; day++) {
    currentWeek.push(day)
    if (currentWeek.length === 7) {
      weeks.push(currentWeek)
      currentWeek = []
    }
  }

  // Fill remaining nulls for days after the last of the month
  while (currentWeek.length < 7 && currentWeek.length > 0) {
    currentWeek.push(null)
  }
  if (currentWeek.length > 0) {
    weeks.push(currentWeek)
  }

  return weeks
}

/**
 * Returns an array of dates in the specified range.
 * @param startDate - The start date of the range (YYYY-MM-DD)
 * @param endDate - The end date of the range (YYYY-MM-DD)
 * @returns An array of dates in the range (YYYY-MM-DD)
 */
export const datesInRange = (startDate: string, endDate: string) => {
  const start = new Date(startDate)
  const end = new Date(endDate)
  const dates: string[] = []

  for (let dt = new Date(start); dt <= end; dt.setDate(dt.getDate() + 1)) {
    dates.push(dt.toISOString().split('T')[0])
  }

  if (!dates.includes(endDate)) {
    dates.push(endDate)
  }

  return dates
}
