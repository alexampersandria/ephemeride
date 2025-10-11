import type { Snippet } from 'svelte'
import type { BlockSize } from '../blocksize'

export type ModalProps = {
  open?: boolean
  children: Snippet
  size?: BlockSize
}
