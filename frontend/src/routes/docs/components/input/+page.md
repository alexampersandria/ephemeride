<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Input from '$lib/components/Input.svelte'
import Message from '$lib/components/Message.svelte'
import Alert from '$lib/components/Alert.svelte'

let value = $state('')
let liveValue = $state('')
let definedValue = $state('Predefined value')
let state = $state('untouched')
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
let cMessages = $state({
  email: '',
  password: []
})
let cValidation = {
  email: (value) => {
    if (!value) {
      cMessages.email = 'This field is required'
      return 'invalid'
    }
    cMessages.email = !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value) ? 'Invalid email address' : ''
    return cMessages.email ? 'invalid' : 'touched'
  },
  password: (value) => {
    cMessages.password = []
    if (value.length < 8) {
      cMessages.password.push('Must be at least 8 characters long')
    }
    if (!/[a-z]/.test(value)) {
      cMessages.password.push('Must contain at least one lowercase letter')
    }
    if (!/[A-Z]/.test(value)) {
      cMessages.password.push('Must contain at least one uppercase letter')
    }
    if (!/[0-9]/.test(value)) {
      cMessages.password.push('Must contain at least one number')
    }
    if (!/[!@#$%^&*]/.test(value)) {
      cMessages.password.push('Must contain at least one special character')
    }
    return cMessages.password.length === 0 ? 'touched' : 'invalid'
  }
}
</script>

# Input

HTML `<input>` element.

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

### Invalid With Feedback Message

An invalid input should have a message to explain the error. This can be done by using the Message component in combination with Input.

<DocsExample left gap="var(--padding-xs)">
  <Input fullwidth value='Joe Smith <email@example.com>' state='invalid' />
  <Message colortext size='small' type='error'>Please enter a valid email address.</Message>
</DocsExample>

```svelte
<Input fullwidth value='Joe Smith <email@example.com>' state='invalid' />
<Message colortext size='small' type='error'>Please enter a valid email address.</Message>
```

### Validation

The input can validate the value using the `validation` prop. The `validation` prop can either be a function that takes in `value` and optionally `state` that returns `InputState`, or a regex that will be tested against the value and return `InputState` based on the result. `invalid` for false and `touched` for true if the value is not empty and the input is not `untouched`.

To return `valid` input state use a custom function as the regex will only return `touched`, `untouched` or `invalid`.

<DocsExample>
  <Input
    validation={/^[a-zA-Z]+$/}
    bind:state={validationRegexState}
    placeholder="Only letters..." />
</DocsExample>
<DocsExample>
  <p>state: <code>{validationRegexState}</code></p>
</DocsExample>

```svelte
<script>
let validationRegexState = $state('untouched')
</script>

<Input
  validation={/^[a-zA-Z]+$/}
  bind:state={validationRegexState}
  placeholder="Only letters..." />
<p>state: <code>{validationRegexState}</code></p>
```

#### Validation and Required

Validation and required can be used together. The input will be required and return `invalid` if the value is empty and state is not `untouched`, otherwise, it will return the result of the validation.

<DocsExample>
  <Input
    required
    validation={/^[a-zA-Z]+$/}
    bind:state={validationRegexStateOther}
    placeholder="Only letters..." />
</DocsExample>
<DocsExample>
  <p>state: <code>{validationRegexStateOther}</code></p>
</DocsExample>

```svelte
<Input
  required
  validation={/^[a-zA-Z]+$/}
  bind:state={validationRegexState}
  placeholder="Only letters..." />
<p>state: <code>{validationRegexState}</code></p>
```

#### Live Validation With Feedback Message

Example of a more complete implementation of an input with live validation and feedback message.

<DocsExample left gap="var(--padding-s)">
  <Input
    bind:value={fValue}
    live
    bind:state={fState}
    required
    fullwidth
    validation={/^[a-zA-Z\- ]+$/}
    placeholder="Your name..." />
  {#if fState === 'invalid'}
    {#if !fValue}
      <Message colortext size='small' type='error'>This field is required.</Message>
    {:else}
      <Message colortext size='small' type='error'>Please enter a valid name.</Message>
    {/if}
  {/if}
</DocsExample>

```svelte
<Input
  bind:value
  live
  bind:state
  required
  fullwidth
  validation={/^[a-zA-Z\- ]+$/}
  placeholder="Your name..." />
{#if fState === 'invalid'}
  {#if !fValue}
    <Message colortext size='small' type='error'>This field is required.</Message>
  {:else}
    <Message colortext size='small' type='error'>Please enter a valid name.</Message>
  {/if}
{/if}
```

#### Specific Validation Feedback Messages

Example of a complete login form with specific validation feedback messages. Password validation will check for at least 8 characters, one lowercase letter, one uppercase letter, one number, and one special character, and provide feedback for each requirement dynamically.

<DocsExample left column>
  <Input
    required
    fullwidth
    bind:value={cValue.email}
    bind:state={cState.email}
    validation={cValidation.email}
    placeholder="Email..." />
  {#if cMessages.email}
    <Message size='small' type='error'>{cMessages.email}</Message>
  {/if}
  <Input
    required
    fullwidth
    type="password"
    bind:value={cValue.password}
    bind:state={cState.password}
    validation={cValidation.password}
    placeholder="Password..." />
  {#if cState.password === 'invalid'}
    <Alert size='small' type='error'>
      <b>Password does not meet the requirements:</b>
      <ul class="text-muted">
        {#each cMessages.password as message}
          <li>{message}</li>
        {/each}
      </ul>
    </Alert>
  {/if}
</DocsExample>

```svelte
<script>
let cValue = $state({
  email: '',
  password: ''
})
let cState = $state({
  email: 'untouched',
  password: 'untouched'
})
let cMessages = $state({
  email: '',
  password: []
})
let cValidation = {
  email: (value) => {
    cMessages.email = !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value) ? 'Invalid email address' : ''
    return cMessages.email ? 'touched' : 'invalid'
  },
  password: (value) => {
    cMessages.password = []
    if (value.length < 8) {
      cMessages.password.push('Must be at least 8 characters long')
    }
    if (!/[a-z]/.test(value)) {
      cMessages.password.push('Must contain at least one lowercase letter')
    }
    if (!/[A-Z]/.test(value)) {
      cMessages.password.push('Must contain at least one uppercase letter')
    }
    if (!/[0-9]/.test(value)) {
      cMessages.password.push('Must contain at least one number')
    }
    if (!/[!@#$%^&*]/.test(value)) {
      cMessages.password.push('Must contain at least one special character')
    }
    return cMessages.password.length === 0 ? 'touched' : 'invalid'
  }
}
</script>

<Input
  required
  fullwidth
  bind:value={cValue.email}
  bind:state={cState.email}
  validation={cValidation.email}
  placeholder="Email..." />
{#if cMessages.email}
  <Message type='error'>{cMessages.email}</Message>
{/if}
<Input
  required
  fullwidth
  type="password"
  bind:value={cValue.password}
  bind:state={cState.password}
  validation={cValidation.password}
  placeholder="Password..." />
{#if cState.password === 'invalid'}
  <Alert type='error'>
    <b>Password does not meet the requirements:</b>
    <ul>
      {#each cMessages.password as message}
        <li>{message}</li>
      {/each}
    </ul>
  </Alert>
{/if}
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
