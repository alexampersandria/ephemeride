<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Entry from '$lib/assemblies/Entry.svelte'
import { defaultCategories } from '$lib/types/log'
</script>

# Entry

<DocsExample>
  <Entry categories={defaultCategories} />
</DocsExample>
