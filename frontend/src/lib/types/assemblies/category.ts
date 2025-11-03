import type { NewTag, Tag } from '../log'

export type CategoryProps = {
  id: string
  name: string
  tags: Tag[]
  selectedTagIds?: string[]
  mode?: 'view' | 'edit' | 'select'
  onAddTag?: (tag: NewTag) => Promise<Tag | null>
  onRemoveTag?: (id: Tag['id']) => Promise<boolean | null>
  onSelectTag?: (tag: Tag, selected: boolean) => Promise<void>
  onEditTag?: (tag: Tag) => Promise<Tag | null>
  onEditCategory?: () => void
}
