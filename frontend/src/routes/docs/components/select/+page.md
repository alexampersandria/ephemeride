<script lang="ts">
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Select from '$lib/components/Select.svelte'

let options = $state([
  { value: '1', label: 'Option 1' },
  { value: '2', label: 'Option 2' },
  { value: '3', label: 'Option 3' },
])

let selected = $state('null')
</script>

## Select

Used in forms or as a dropdown menu to select an option from a list. Modifies the selected prop via `$bindable`.

<DocsExample>
  <Select
    bind:selected
    bind:options
  />
  <p style="margin-bottom:0;">
    Selected: <code>{selected}</code>
  </p>
</DocsExample>

```svelte
<script>
let options = $state([
  { value: '1', label: 'Option 1' },
  { value: '2', label: 'Option 2' },
  { value: '3', label: 'Option 3' },
])

let selected = $state('null')
</script>

<Select
  bind:selected
  bind:options
/>
<p>
  Selected: <code>{selected}</code>
</p>
```

