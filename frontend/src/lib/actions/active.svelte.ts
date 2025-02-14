import { isActiveRoute } from '$lib/utils/routes.svelte'

export const active = (node: HTMLElement) => {
  $effect(() => {
    const route = node.getAttribute('href')
    if (route) {
      if (isActiveRoute(route)) {
        node.classList.add('active')
      }
      return () => {
        node.classList.remove('active')
      }
    }
  })
}
