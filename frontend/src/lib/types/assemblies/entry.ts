import type { MoodValue } from '../components/moodinput'

export type EntryProps = {
  /**
   * mood value
   * number between 1-5
   */
  mood?: MoodValue
  /**
   * selected tag ids
   */
  tagsIds?: string[]
}
