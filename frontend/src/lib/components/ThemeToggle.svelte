<script lang="ts">
import { useUiStore } from '$lib/store/uiStore.svelte'
import { Moon, Sun } from 'lucide-svelte'
import Button from './Button.svelte'

let uiStore = useUiStore()

const themeToggle = () => {
  if (uiStore.appliedTheme === 'dark') {
    uiStore.theme = 'light'
  } else {
    uiStore.theme = 'dark'
  }
}

const ThemeToggleIcon = $derived.by(() => {
  if (uiStore.appliedTheme === 'dark') {
    return Sun
  } else {
    return Moon
  }
})
</script>

<Button type="ghost" onclick={themeToggle} aria-label="Toggle Theme">
  <ThemeToggleIcon />
</Button>
