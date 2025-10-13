import type { MoodValue } from '../components/moodinput'
import type { CategoryWithTags } from '../log'

export type EntryProps = {
  /**
   * date string in YYYY-MM-DD format
   */
  date: string
  /**
   * component mode
   * 'view' | 'edit' | 'create'
   */
  mode?: 'view' | 'edit' | 'create'
  /**
   * mood value
   * number between 1-5
   */
  mood?: MoodValue
  /**
   * selected tag ids
   */
  categories?: CategoryWithTags[]
  /**
   * entry text
   */
  entry?: string
  /**
   * array of selected tag ids
   */
  selectedTagIds?: string[]
}
