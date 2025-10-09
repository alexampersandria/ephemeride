<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Textarea from '$lib/components/Textarea.svelte'

let value = $state('')
let definedValue = $state('Predefined value')
let inputstate = $state('untouched')
</script>

# Textarea

HTML `<textarea>` element.

## Usage

Used in forms to collect multi-line user input. Modifies the value prop via `$bindable`.

### Value

The value of the textarea is bound to the `value` prop. The value will be modified on input.

<DocsExample>
  <Textarea placeholder="Your thoughts..." bind:value />
</DocsExample>
<DocsExample>
  <p>value: <code>{value}</code></p>
</DocsExample>

```svelte
<script>
let value = $state('')
</script>

<Textarea placeholder="Your thoughts..." bind:value /><p>value: <code>{value}</code></p>
```

### Predefined Value

The value can be predefined by setting the `value` prop.

<DocsExample>
  <Textarea bind:value={definedValue} />
</DocsExample>

```svelte
<script>
let definedValue = $state('Predefined value')
</script>

<Textarea bind:value={definedValue} />
```

### Disabled

The textarea can be disabled using the HTML `disabled` attribute. Disabled textareas should generally be avoided if at all possible as they can be confusing to users, and only used when the textarea can be re-enabled by a user action in the same context.

<DocsExample>
  <Textarea disabled placeholder="Your thoughts..." />
</DocsExample>

```svelte
<Textarea disabled placeholder="Your thoughts..." />
```

### Full Width

The textarea can be set to full width using the `fullwidth` prop.

<DocsExample>
  <Textarea fullwidth placeholder="Full width textarea..." />
</DocsExample>

```svelte
<Textarea fullwidth placeholder="Full width textarea..." />
```

### Max Length

The textarea can enforce a maximum character limit using the `maxlength` prop. When set, a character counter will be displayed and the textarea will automatically validate the input. The counter updates live as you type.

<DocsExample>
  <Textarea maxlength={50} fullwidth placeholder="Max 50 characters..." />
</DocsExample>

```svelte
<Textarea maxlength={50} fullwidth placeholder="Max 50 characters..." />
```

If the text exceeds the maximum length, the textarea will be marked as invalid and the counter will turn red.

<DocsExample>
  <Textarea maxlength={20} value="This text is way too long and exceeds the maximum" fullwidth />
</DocsExample>

```svelte
<Textarea maxlength={20} value="This text is way too long and exceeds the maximum" fullwidth />
```

### Input State

Input state can be set to `touched`, `untouched`, or `invalid`. If not set, the default state is `untouched`. Input state uses `$bindable` to modify the inputstate prop.

See [Input Types](/docs/types/Input) for more information on input states.

<DocsExample>
  <Textarea bind:inputstate placeholder="Your thoughts..." />
</DocsExample>
<DocsExample>
  <p style="margin: 0;">inputstate: <code>{inputstate}</code></p>
</DocsExample>

```svelte
<script>
let inputstate = $state('untouched')
</script>

<Textarea bind:inputstate placeholder="Your thoughts..." /><p>inputstate: <code>{inputstate}</code></p>
```

### Required

The textarea can be set to required using the HTML `required` attribute.

<DocsExample>
  <Textarea required placeholder="Your thoughts..." />
</DocsExample>

```svelte
<Textarea required placeholder="Your thoughts..." />
```

## Types

### Props

Inherits `FormElementProps`.

| Name        | Type         | Required | Default     | Description                                                |
| ----------- | ------------ | :------: | ----------- | ---------------------------------------------------------- |
| value       | `string`     |          |             | Value of the textarea.                                     |
| placeholder | `string`     |          |             | Placeholder text.                                          |
| fullwidth   | `boolean`    |          | `false`     | Full width textarea.                                       |
| maxlength   | `number`     |          |             | Maximum character limit. Shows counter and validates live. |
| disabled    | `boolean`    |          | `false`     | Disables the textarea. Inherited from `FormElementProps`.  |
| inputstate  | `InputState` |          | `untouched` | State of the textarea. Inherited from `FormElementProps`.  |
| name        | `string`     |          |             | Name of the textarea. Inherited from `FormElementProps`.   |
| id          | `string`     |          |             | ID of the textarea. Inherited from `FormElementProps`.     |
| required    | `boolean`    |          | `false`     | Required attribute. Inherited from `FormElementProps`.     |

## References

- [InputState](/docs/types/input#inputstate)
- [FormElementProps](/docs/types/input#formelementprops)
