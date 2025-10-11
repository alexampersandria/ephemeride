import type { Tag } from '../log'

export type CategoryProps = {
  name: string
  tags: Tag[]
  selectedTagIds: string[]
  mode?: 'view' | 'edit' | 'select'
  onaddtag?: (tag: Tag) => void
}
