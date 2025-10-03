import type { Snippet } from "svelte"
import type { Color } from "../color"

export type ChipVariant = 'subtle' | 'solid'

export type ChipProps = {
    children: Snippet
    color: Color
    variant?: ChipVariant
}