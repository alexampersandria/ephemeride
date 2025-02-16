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
let state = $state('untouched')
</script>

# Select

Default HTML `<select>` element with `<option>`'s.

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

### Placeholder

Placeholder text can be set to display when no option is selected. The placeholder is rendered as a disabled option at the top of the list which is selected if no value is set.

<DocsExample>
  <Select {options} placeholder="Select an option" />
</DocsExample>

```svelte
<Select {options} placeholder="Select an option" />
```

Placeholder can also be included even when a value is set.

<DocsExample>
  <Select bind:value={otherValue} {options} placeholder="Select an option" />
</DocsExample>

```svelte
<script>
let value = $state('2')
</script>

<Select bind:value {options} placeholder="Select an option" />
```

### State

State can be set to `touched`, `untouched`, `valid`, or `invalid`. If not set, the default state is `untouched`. State uses `$bindable` to modify the state prop, on change will set state to `touched` if state is `untouched`.

See [Input Types](/docs/types/Input) for more information on states.

<DocsExample>
  <Select {options} bind:state />
</DocsExample>
<DocsExample>
  <p style="margin: 0;">state: <code>{state}</code></p>
</DocsExample>

```svelte
<script>
let state = $state('untouched')
</script>

<Select {options} bind:state />
<p>state: <code>{state}</code></p>
```

State can be set to `valid` or `invalid`.

<DocsExample>
  <Select {options} state='valid' />
  <Select {options} state='invalid' />
</DocsExample>

```svelte
<Select {options} state="valid" />
<Select {options} state="invalid" />
```

### Disabled

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

### Name and ID

The `name` and `id` attributes can be set to identify the select in a form.

<DocsExample>
  <Select {options} name="select" id="select" />
</DocsExample>

```svelte
<Select {options} name="select" id="select" />
```

### Full Width

The select can be set to full width of parent using the `fullwidth` prop.

<DocsExample>
  <Select {options} fullwidth />
</DocsExample>

```svelte
<Select {options} fullwidth />
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
| state       | `InputState`   |          | `untouched` | State of the select. Inherited from `FormElementProps`. |
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
