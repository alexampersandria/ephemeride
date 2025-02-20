<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import FormField from '$lib/components/FormField.svelte'
import Label from '$lib/components/Label.svelte'
import Input from '$lib/components/Input.svelte'
import Select from '$lib/components/Select.svelte'
</script>

# FormField

The FormField component is a wrapper for form elements that includes a label and any input component such as [Input](/docs/components/input), [Select](/docs/components/select), etc.

## Usage



### ID

FormFields automatically generate a unique ID for the label and input element. The ID is passed to the children snippet as a prop, and can be used in the children to associate the label with the input element using the `for` and `id` attributes.

<DocsExample>
  <form>
    <FormField>
      {#snippet children(id)}
        <Label for={id}>Name</Label>
        <Input {id} />
      {/snippet}
    </FormField>
    <FormField>
      {#snippet children(id)}
        <Label for={id}>Favourite Frontend Library</Label>
        <Select {id} placeholder='Choose one...' options={[
          { value: 'svelte', label: 'Svelte' },
          { value: 'vue', label: 'Vue' },
          { value: 'react', label: 'React (Incorrect)' },
          { value: 'angular', label: 'Angular' }
        ]} />
      {/snippet}
    </FormField>
  </form>
</DocsExample>

```svelte
<form>
  <FormField>
    {#snippet children(id)}
      <Label for={id}>Name</Label>
      <Input {id} />
    {/snippet}
  </FormField>
  <FormField>
    {#snippet children(id)}
      <Label for={id}>Favourite Frontend Library</Label>
      <Select {id} placeholder='Choose one...' options={[
        { value: 'svelte', label: 'Svelte' },
        { value: 'vue', label: 'Vue' },
        { value: 'react', label: 'React (Incorrect)' },
        { value: 'angular', label: 'Angular' }
      ]} />
    {/snippet}
  </FormField>
</form>
```
  