<script>
import Label from '$lib/components/Label.svelte'
import Input from '$lib/components/Input.svelte'
import DocsExample from '$lib/components/utils/DocsExample.svelte'
</script>

# Label

Labels are used in combination with input components to provide context for the user.

## Usage

Should almost always be used inside a [FormField](/docs/components/form-field).

<DocsExample>
  <Label for='name'>Name</Label>
  <Input id='name' />
</DocsExample>

```svelte
<Label for='name'>Name</Label>
<Input id='name' />
```
