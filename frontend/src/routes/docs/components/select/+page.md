<script lang="ts">
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Select from '$lib/components/Select.svelte'

let options = $state([
  { value: '1', label: 'Option 1' },
  { value: '2', label: 'Option 2' },
  { value: '3', label: 'Option 3' },
])

let value = $state('null')
</script>

## Select

Default HTML `<select>` element with `<option>`'s.

Used in forms or as a dropdown menu to select an option from a list. Modifies the value prop via `$bindable`.

<DocsExample>
  <Select bind:value {options} />
  <p style="margin-bottom:0;">value: <code>{value}</code></p>
</DocsExample>

```svelte
<script>
let options = $state([
  { value: '1', label: 'Option 1' },
  { value: '2', label: 'Option 2' },
  { value: '3', label: 'Option 3' },
])

let value = $state('null')
</script>

<Select bind:value {options} />
<p>value: <code>{value}</code></p>
```

