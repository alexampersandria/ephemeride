/**
 * mode of determining the functionality of the calendar
 */
export type CalendarMode = 'navigation' | 'datepicker'

/**
 * props for the Calendar component
 */
export type CalendarProps = {
  mode?: CalendarMode
  month?: number
  year?: number
}
