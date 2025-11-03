import type { MoodValue } from '../components/moodinput'
import type {
  Category,
  CategoryWithTags,
  EditCategory,
  EditEntry,
  EditTag,
  Entry,
  NewCategory,
  NewEntry,
  NewTag,
  Tag,
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
  onCreate?: (entry: NewEntry) => Promise<Entry | null>
  /**
   * called when an existing entry is updated
   */
  onUpdate?: (entry: EditEntry) => Promise<Entry | null>
  /**
   * called when an existing entry is deleted
   */
  onDelete?: (id: string) => Promise<boolean | null>
  /**
   * when category component emits onaddtag
   */
  onAddTag?: (tag: NewTag) => Promise<Tag | null>
  /**
   * when category component emits onedittag
   */
  onEditTag?: (tag: EditTag) => Promise<Tag | null>
  /**
   * when category component emits onremovetag
   */
  onRemoveTag?: (id: string) => Promise<boolean | null>
  /**
   * when a new category is created
   */
  onAddCategory?: (category: NewCategory) => Promise<Category | null>
  /**
   * when an existing category is edited
   */
  onEditCategory?: (category: EditCategory) => Promise<Category | null>
  /**
   * when a category is deleted
   */
  onDeleteCategory?: (id: string) => Promise<boolean | null>
}
