<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Input from '$lib/components/Input.svelte'
import Message from '$lib/components/Message.svelte'

let value = $state('')
let liveValue = $state('')
let definedValue = $state('Predefined value')
let state = $state('untouched')
</script>

# Input

Default HTML `<input>` element.

## Usage

Used in forms to collect user input. Modifies the value prop via `$bindable`.

### Value

The value of the input is bound to the `value` prop. The value will be modified on change.

<DocsExample>
  <Input bind:value />
</DocsExample>
<DocsExample>
  <p>value: <code>{value}</code></p>
</DocsExample>

```svelte
<script>
let value = $state('')
</script>

<Input bind:value />
<p>value: <code>{value}</code></p>
```

It is possible to set a predefined value.

<DocsExample>
  <Input bind:value={definedValue} />
</DocsExample>
<DocsExample>
  <p>value: <code>{definedValue}</code></p>
</DocsExample>

```svelte
<script>
let definedValue = $state('Predefined value')
</script>

<Input bind:value={definedValue} />
<p>value: <code>{definedValue}</code></p>
```

### Live

The value of the input can be set to update live using the `live` prop. The value will be modified on input instead of change.

<DocsExample>
  <Input bind:value={liveValue} live />
</DocsExample>
<DocsExample>
  <p>value: <code>{liveValue}</code></p>
</DocsExample>

```svelte
<script>
let liveValue = $state('')
</script>

<Input bind:value={liveValue} live />
<p>value: <code>{liveValue}</code></p>
```

### Placeholder

Placeholder text can be set to display when no value is set using the HTML `placeholder` attribute.

<DocsExample>
  <Input placeholder="Your name..." />
</DocsExample>

```svelte
<Input placeholder="Your name..." />
```

### Disabled

The input can be disabled using the HTML `disabled` attribute.

<DocsExample>
  <Input disabled placeholder="Your name..." />
</DocsExample>

```svelte
<Input disabled placeholder="Your name..." />
```

### State

State can be set to `touched`, `untouched`, `valid`, or `invalid`. If not set, the default state is `untouched`. State uses `$bindable` to modify the state prop, on change will set state to `touched` if state is `untouched`.

See [Input Types](/docs/types/Input) for more information on states.

<DocsExample>
  <Input bind:state placeholder="Your name..." />
</DocsExample>
<DocsExample>
  <p style="margin: 0;">state: <code>{state}</code></p>
</DocsExample>

```svelte
<script>
let state = $state('untouched')
</script>

<Input bind:state placeholder="Your name..." />
<p>state: <code>{state}</code></p>
```

State can be set to `valid` or `invalid`.

<DocsExample>
  <Input value='claire@example.com' state='valid' />
  <Input value='Joe Smith <email@example.com>' state='invalid' />
</DocsExample>

```svelte
<Input value='claire@example.com' state='valid' />
<Input value='Joe Smith <email@example.com>' state='invalid' />
```

#### Invalid With Feedback Message

An invalid input should have a message to explain the error. This can be done by using the Message component in combination with Input.

<DocsExample left gap="var(--padding-xs)">
  <Input fullwidth value='Joe Smith <email@example.com>' state='invalid' />
  <Message colortext size='small' type='error'>Please enter a valid email address.</Message>
</DocsExample>

```svelte
<Input fullwidth value='Joe Smith <email@example.com>' state='invalid' />
<Message colortext size='small' type='error'>Please enter a valid email address.</Message>
```

### Required

The input can be set to required using the HTML `required` attribute. Required elements will change state to `invalid` if empty on change.

<DocsExample>
  <Input required placeholder="Your name..." />
</DocsExample>

```svelte
<Input required placeholder="Your name..." />
```

### Name and ID

The input can be set to required using the HTML `name` and `id` attributes, this can be used for labels and form submission.

<DocsExample>
  <Input name="name" id="name" placeholder="Your name..." />
</DocsExample>

```svelte
<Input name="name" id="name" placeholder="Your name..." />
```

### Type

The input type can be set using the HTML `type` attribute. The default type is `text`.

<DocsExample>
  <Input type="password" placeholder="Password..." />
</DocsExample>

```svelte
<Input type="password" placeholder="Password..." />
```

### Full Width

The input can be set to full width of parent using the `fullwidth` prop.

<DocsExample>
  <Input fullwidth placeholder="Full width input..." />
</DocsExample>

```svelte
<Input fullwidth placeholder="Full width input..." />
```

## Types

### Props

Inherits `FormElementProps`.

| Name        | Type         | Required | Default     | Description                                            |
| ----------- | ------------ | :------: | ----------- | ------------------------------------------------------ |
| type        | `string`     |          | `'text'`    | HTML input type.                                       |
| value       | `string`     |          |             | Value of the input.                                    |
| placeholder | `string`     |          |             | Placeholder text.                                      |
| fullwidth   | `boolean`    |          | `false`     | Full width input.                                      |
| live        | `boolean`    |          | `false`     | Live input update.                                     |
| disabled    | `boolean`    |          | `false`     | Disables the input. Inherited from `FormElementProps`. |
| state       | `InputState` |          | `untouched` | State of the input. Inherited from `FormElementProps`. |
| name        | `string`     |          |             | Name of the input. Inherited from `FormElementProps`.  |
| id          | `string`     |          |             | ID of the input. Inherited from `FormElementProps`.    |
| required    | `boolean`    |          | `false`     | Required attribute. Inherited from `FormElementProps`. |

## References

- [InputState](/docs/types/input#inputstate)
- [FormElementProps](/docs/types/input#formelementprops)
