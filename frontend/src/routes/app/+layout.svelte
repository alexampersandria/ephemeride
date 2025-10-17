<script>
import { goto } from '$app/navigation'
import { useUserStore } from '$lib/store/userStore.svelte'
import { onMount } from 'svelte'

let { children } = $props()

let userStore = useUserStore()

const title = 'Ephemeride'

onMount(() => {
  if (!userStore.sessionId) {
    goto('/')
  }
})
</script>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<div class="app-wrapper">
  {@render children()}
</div>
