<script lang="ts">
import { page } from '$app/state'
import { compareRoutes } from '$lib/utils/routes'

let { href, children } = $props()
let isActive = $derived(compareRoutes(page.route.id || '', href))
</script>

<a class="internal-link {isActive ? 'active' : ''}" {href}>
  {@render children()}
</a>
