import { browser } from '$app/environment'

export type UiState = {
  theme: 'DARK' | 'LIGHT' | 'SYSTEM'
}

export const uiState = $state({
  theme: 'SYSTEM',
})

export const theme = () => {
  if (uiState.theme === 'SYSTEM' && browser) {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
  } else {
    return uiState.theme.toLowerCase()
  }
}
