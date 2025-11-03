import type { MoodValue } from '../components/moodinput'
import type {
  CategoryWithTags,
  EditCategory,
  EditEntry,
  EditTag,
  NewCategory,
  NewEntry,
  NewTag,
} from '../log'

export type EntryProps = {
  /**
   * entry id
   */
  id?: string
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

  /**
   * called when a new entry is created
   */
  onCreate?: (entry: NewEntry) => void
  /**
   * called when an existing entry is updated
   */
  onUpdate?: (entry: EditEntry) => void
  /**
   * called when an existing entry is deleted
   */
  onDelete?: (id: string) => void
  /**
   * when category component emits onaddtag
   */
  onAddTag?: (tag: NewTag) => void
  /**
   * when category component emits onremovetag
   */
  onRemoveTag?: (id: string) => void
  /**
   * when category component emits onedittag
   */
  onEditTag?: (tag: EditTag) => void
  /**
   * when a new category is created
   */
  onAddCategory?: (category: NewCategory) => void
  /**
   * when an existing category is edited
   */
  onEditCategory?: (category: EditCategory) => void
  /**
   * when a category is deleted
   */
  onDeleteCategory?: (id: string) => void
}
