import type { NewTag, Tag } from '../log'

export type CategoryProps = {
  name: string
  tags: Tag[]
  selectedTagIds: string[]
  mode?: 'view' | 'edit' | 'select'
  onaddtag?: (tag: NewTag) => void
  onremovetag?: (id: Tag['id']) => void
  onselecttag?: (tag: Tag, selected: boolean) => void
  onedittag?: (tag: Tag) => void
}
