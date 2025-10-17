import { browser } from '$app/environment'
import { onMount } from 'svelte'

export const themes = ['dark', 'light', 'system'] as const
export type Theme = (typeof themes)[number]

export type UiState = {
  theme: Theme
  loading: boolean
  leftMenuOpen: boolean
  leftMenuWidth: number
  appliedTheme: Theme
}

const mountedAt: number = new Date().getTime()

let theme: Theme = $state('system')
let loading = $state(true)
let leftMenuOpen = $state(false)
let leftMenuWidth = $state(320)

const appliedTheme: Theme = $derived.by(() => {
  if (browser) {
    if (theme === 'system') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light'
    }
    return theme
  }
  return 'dark'
})

export const useUiStore: () => UiState = () => {
  onMount(() => {
    if (browser) {
      // wait min 300ms before setting loading to false, less if time difference between mountedAt and now is less
      // if preloader is shown for less than 300ms it may not register to the user
      const minWait = 1000
      const timeDiff = new Date().getTime() - mountedAt
      const timeout = timeDiff > minWait ? 0 : minWait - timeDiff
      setTimeout(() => {
        loading = false
      }, timeout)
    }
  })

  return {
    get theme() {
      return theme
    },
    set theme(value) {
      theme = value
    },
    get loading() {
      return loading
    },
    get leftMenuOpen() {
      return leftMenuOpen
    },
    set leftMenuOpen(value) {
      leftMenuOpen = value
    },
    get leftMenuWidth() {
      return leftMenuWidth
    },
    set leftMenuWidth(value) {
      leftMenuWidth = value
    },
    get appliedTheme() {
      return appliedTheme
    },
  }
}
