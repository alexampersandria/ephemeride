<script>
import Label from '$lib/components/Label.svelte'
import Input from '$lib/components/Input.svelte'
import DocsExample from '$lib/components/utils/DocsExample.svelte'
</script>

# Label

Labels are used in combination with input components to provide context for the user.

## Usage

Should almost always be used alongside an input component. The `for` prop should match the `id` of the input element to ensure accessibility.

<DocsExample>
  <div class="form-field">
    <Label for='name'>Name</Label>
    <Input id='name' />
  </div>
</DocsExample>

```svelte
<div class="form-field">
  <Label for='name'>Name</Label>
  <Input id='name' />'
</div>
```

## Types

### Props

| Name | Type     | Required | Default | Description                                   |
| ---- | -------- | :------: | ------- | --------------------------------------------- |
| for  | `string` |          |         | The ID of the input element the label is for. |
