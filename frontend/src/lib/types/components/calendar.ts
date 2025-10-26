import type { Color } from '../color'

/**
 * mode of determining the functionality of the calendar
 */
export type CalendarMode = 'single' | 'range' | 'multiple'
/**
 * a day in the calendar with an optional color, used for stats
 */
export type CalendarDay = {
  date: string
  color: Color | null
}

/**
 * props for the Calendar component
 */
export type CalendarProps = {
  mode?: CalendarMode
  month?: number
  year?: number
  days?: CalendarDay[]
}
