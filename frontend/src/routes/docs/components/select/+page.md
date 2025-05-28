<script lang="ts">
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Select from '$lib/components/Select.svelte'

let options = $state([
  { value: '1', label: 'Option 1' },
  { value: '2', label: 'Option 2' },
  { value: '3', label: 'Option 3' },
])

let optionsWithDisabled = $state([
  { value: '1', label: 'Option 1' },
  { value: '2', label: 'Option 2', disabled: true },
  { value: '3', label: 'Option 3' },
])

let value = $state('')
let otherValue = $state('2')
let inputstate = $state('untouched')
</script>

# Select

HTML `<select>` element with `<option>`'s.

## Usage

Used in forms or as a dropdown menu to select an option from a list. Modifies the value prop via `$bindable`.

### Options & Value

Options are an array of objects with `value` and `label` properties. The `value` is the value of the option and the `label` is the text displayed in the dropdown. The `value` of the selected option is stored in the `value` prop.

<DocsExample>
  <Select bind:value {options} placeholder="Select an option" />
</DocsExample>
<DocsExample>
  <p style="margin: 0;">value: <code>{value}</code></p>
</DocsExample>

```svelte
<script>
let options = $state([
  { value: '1', label: 'Option 1' },
  { value: '2', label: 'Option 2' },
  { value: '3', label: 'Option 3' },
])

let value = $state('')
</script>

<Select bind:value {options} placeholder="Select an option" />
<p>value: <code>{value}</code></p>
```

### Input State

Input state can be set to `touched`, `untouched`, or `invalid`. If not set, the default state is `untouched`. Input state uses `$bindable` to modify the state prop, on change will set state to `touched` if state is `untouched`.

See [Input Types](/docs/types/Input) for more information on input states.

<DocsExample>
  <Select {options} bind:inputstate />
</DocsExample>
<DocsExample>
  <p style="margin: 0;">inputstate: <code>{inputstate}</code></p>
</DocsExample>

```svelte
<script>
let inputstate = $state('untouched')
</script>

<Select {options} bind:inputstate />
<p>inputstate: <code>{inputstate}</code></p>
```

State can be set to `invalid`.

<DocsExample>
  <Select {options} inputstate='invalid' />
</DocsExample>

```svelte
<Select {options} inputstate="invalid" />
```

### Disabled

The select can be disabled using the HTML `disabled` attribute. Disabled input elements such as select should generally be avoided if at all possible as they can be confusing to users, and only used when the select can be re-enabled by a user action in the same context.

<DocsExample>
  <Select {options} disabled />
</DocsExample>

```svelte
<Select {options} disabled />
```

Indivial options can also be disabled.

<DocsExample>
  <Select options={optionsWithDisabled} />
</DocsExample>

```svelte
<script>
let optionsWithDisabled = $state([
  { value: '1', label: 'Option 1' },
  { value: '2', label: 'Option 2', disabled: true },
  { value: '3', label: 'Option 3' },
])
</script>

<Select options={optionsWithDisabled} />
```

## Types

### Props

Inherits `FormElementProps`.

| Name        | Type           | Required | Default     | Description                                             |
| ----------- | -------------- | :------: | ----------- | ------------------------------------------------------- |
| options     | `SelectOption` |    ✅     |             | Array of objects with `value` and `label` properties.   |
| value       | `string`       |          |             | Value of the selected option.                           |
| placeholder | `string`       |          |             | Placeholder text.                                       |
| fullwidth   | `boolean`      |          | `false`     | Full width select.                                      |
| disabled    | `boolean`      |          | `false`     | Disables the select. Inherited from `FormElementProps`. |
| inputstate  | `InputState`   |          | `untouched` | State of the select. Inherited from `FormElementProps`. |
| name        | `string`       |          |             | Name of the select. Inherited from `FormElementProps`.  |
| id          | `string`       |          |             | ID of the select. Inherited from `FormElementProps`.    |
| required    | `boolean`      |          | `false`     | Required attribute. Inherited from `FormElementProps`.  |

### SelectOption

| Name     | Type      | Required | Default | Description          |
| -------- | --------- | :------: | ------- | -------------------- |
| value    | `string`  |    ✅     |         | Value of the option. |
| label    | `string`  |    ✅     |         | Label of the option. |
| disabled | `boolean` |          | `false` | Disables the option. |

## References

- [InputState](/docs/types/input#inputstate)
- [FormElementProps](/docs/types/input#formelementprops)
