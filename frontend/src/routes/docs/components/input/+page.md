<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Input from '$lib/components/Input.svelte'
import Message from '$lib/components/Message.svelte'
import Alert from '$lib/components/Alert.svelte'

import { validateEmail, validatePassword } from '$lib/utils/validationRules'

let value = $state('')
let liveValue = $state('')
let definedValue = $state('Predefined value')
let inputstate = $state('untouched')
let validationRegexState = $state('untouched')
let validationFunctionState = $state('untouched')
let validationRegexStateOther = $state('untouched')

let fValue = $state('')
let fState = $state('untouched')

let cValue = $state({
  email: '',
  password: ''
})
let cState = $state({
  email: 'untouched',
  password: 'untouched'
})
let messages = $state({
  email: [],
  password: []
})
</script>

# Input

HTML `<input>` element.

## Usage

Used in forms to collect user input. Modifies the value prop via `$bindable`.

### Value

The value of the input is bound to the `value` prop. The value will be modified on change.

<DocsExample>
  <Input placeholder="Your name..." bind:value />
</DocsExample>
<DocsExample>
  <p>value: <code>{value}</code></p>
</DocsExample>

```svelte
<script>
let value = $state('')
</script>

<Input placeholder="Your name..." bind:value />
<p>value: <code>{value}</code></p>
```

### Disabled

The input can be disabled using the HTML `disabled` attribute. Disabled inputs should generally be avoided if at all possible as they can be confusing to users, and only used when the input can be re-enabled by a user action in the same context.

<DocsExample>
  <Input disabled placeholder="Your name..." />
</DocsExample>

```svelte
<Input disabled placeholder="Your name..." />
```

### Input State

Input state can be set to `touched`, `untouched`, or `invalid`. If not set, the default state is `untouched`. Input state uses `$bindable` to modify the inputstate prop, on change will set state to `touched` if state is `untouched`.

See [Input Types](/docs/types/Input) for more information on input states.

<DocsExample>
  <Input bind:inputstate placeholder="Your name..." />
</DocsExample>
<DocsExample>
  <p style="margin: 0;">inputstate: <code>{inputstate}</code></p>
</DocsExample>

```svelte
<script>
let inputstate = $state('untouched')
</script>

<Input bind:inputstate placeholder="Your name..." />
<p>inputstate: <code>{inputstate}</code></p>
```

State can be set to `invalid` to indicate an error in the input.

<DocsExample>
  <Input value='Joe Smith <email@example.com>' inputstate='invalid' />
</DocsExample>

```svelte
<Input value='Joe Smith <email@example.com>' inputstate='invalid' />
```

### Required

The input can be set to required using the HTML `required` attribute. Required elements will change state to `invalid` if empty on change.

<DocsExample>
  <Input required placeholder="Your name..." />
</DocsExample>

```svelte
<Input required placeholder="Your name..." />
```

### Validation

The input can validate the value using the `validation` prop. The `validation` prop can either be a function that takes in `value` and optionally `inputstate` that returns `InputState`, or a regex that will be tested against the value and return `InputState` based on the result. `invalid` for false and `touched` for true if the value is not empty and the input is not `untouched`.

<DocsExample>
  <Input
    validation={/^[a-zA-Z]+$/}
    bind:inputstate={validationRegexState}
    placeholder="Only letters..." />
</DocsExample>
<DocsExample>
  <p>inputstate: <code>{validationRegexState}</code></p>
</DocsExample>

```svelte
<script>
let validationRegexState = $state('untouched')
</script>

<Input
  validation={/^[a-zA-Z]+$/}
  bind:inputstate={validationRegexState}
  placeholder="Only letters..." />
<p>inputstate: <code>{validationRegexState}</code></p>
```

Validation and required can be used together. The input will be required and return `invalid` if the value is empty and state is not `untouched`, otherwise, it will return the result of the validation.

<DocsExample>
  <Input
    required
    validation={/^[a-zA-Z]+$/}
    bind:inputstate={validationRegexStateOther}
    placeholder="Only letters and required..." />
</DocsExample>
<DocsExample>
  <p>inputstate: <code>{validationRegexStateOther}</code></p>
</DocsExample>

```svelte
<Input
  required
  validation={/^[a-zA-Z]+$/}
  bind:inputstate={validationRegexState}
  placeholder="Only letters and required..." />
<p>inputstate: <code>{validationRegexState}</code></p>
```

#### Feedback Messages

An invalid input should have a message to explain the error. This can be done by using the Message component in combination with Input.

<DocsExample column left gap="var(--padding-xs)">
  <Input
    fullwidth
    value='Joe Smith <email@example.com>'
    inputstate='invalid' />
  <Message size='small' type='error'>
    Please enter a valid email address.
  </Message>
</DocsExample>

```svelte
<Input
  fullwidth
  value='Joe Smith <email@example.com>'
  inputstate='invalid' />
<Message size='small' type='error'>
  Please enter a valid email address.
</Message>
```

#### Using Validation Rules

Using [Validation Rules](#TODO) to validate the input.

<DocsExample left column>
  <Input
    required
    fullwidth
    validation={value => {
      return validateEmail(value, errors => {
        messages.email = errors
      })
    }}
    placeholder="Email..." />
  {#each messages.email as message}
    <Message size='small' type='error'>{message}</Message>
  {/each}
  <Input
    required
    fullwidth
    type="password"
    validation={value => {
      return validatePassword(value, errors => {
        messages.password = errors
      })
    }}
    placeholder="Password..." />
  {#if messages.password.length > 0}
    <Alert size='small' type='error'>
      <b>Password does not meet the requirements:</b>
      <ul>
        {#each messages.password as message}
          <li>{message}</li>
        {/each}
      </ul>
    </Alert>
  {/if}
</DocsExample>

```svelte
<script>
import { validateEmail, validatePassword } from '$lib/utils/validationRules'

let messages = $state({
  email: [],
  password: []
})
</script>

<Input
  required
  fullwidth
  validation={value => {
    return validateEmail(value, errors => {
      messages.email = errors
    })
  }}
  placeholder="Email..." />
{#each messages.email as message}
  <Message size='small' type='error'>{message}</Message>
{/each}
<Input
  required
  fullwidth
  type="password"
  validation={value => {
    return validatePassword(value, errors => {
      messages.password = errors
    })
  }}
  placeholder="Password..." />
{#if messages.password.length > 0}
  <Alert size='small' type='error'>
    <b>Password does not meet the requirements:</b>
    <ul>
      {#each messages.password as message}
        <li>{message}</li>
      {/each}
    </ul>
  </Alert>
{/if}
```

## Types

### Props

Inherits `FormElementProps`.

| Name        | Type             | Required | Default     | Description                                            |
| ----------- | ---------------- | :------: | ----------- | ------------------------------------------------------ |
| type        | `string`         |          | `'text'`    | HTML input type.                                       |
| value       | `string`         |          |             | Value of the input.                                    |
| placeholder | `string`         |          |             | Placeholder text.                                      |
| fullwidth   | `boolean`        |          | `false`     | Full width input.                                      |
| live        | `boolean`        |          | `false`     | Live input update.                                     |
| disabled    | `boolean`        |          | `false`     | Disables the input. Inherited from `FormElementProps`. |
| inputstate  | `InputState`     |          | `untouched` | State of the input. Inherited from `FormElementProps`. |
| validation  | `ValidationRule` |          |             | Validation rule for the input.                         |
| name        | `string`         |          |             | Name of the input. Inherited from `FormElementProps`.  |
| id          | `string`         |          |             | ID of the input. Inherited from `FormElementProps`.    |
| required    | `boolean`        |          | `false`     | Required attribute. Inherited from `FormElementProps`. |

## References

- [InputState](/docs/types/input#inputstate)
- [ValidationRule](/docs/types/input#validationrule)
- [FormElementProps](/docs/types/input#formelementprops)
