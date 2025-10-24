# Input Types

## ButtonType

The type of the button. Default should always be `'secondary'`.

- `primary` should be used for highlighted actions, such as confirmation in a dialogue or submitting a form.
- `secondary` should be used for most other actions.
- `ghost` should be used the same as `secondary` for aesthetic purposes where it fits better.
- `destructive` should be used for actions that will delete or remove data and should be used sparingly.

### Definition

```ts
type ButtonType = 'primary' | 'secondary' | 'ghost' | 'destructive' | 'navigation'
```

## InputState

The state of the input, default should usually be `untouched`, components should automatically set to `touched` if `untouched` when interacted with via `$bindable`. `touched` state is equats to a valid input state.

### Defition

```ts
type InputState = 'untouched' | 'touched' | 'invalid'
```

## FormElementProps

Shared props for form elements such as inputs, selects, and textareas.

| Name       | Type         | Required | Default     | Description         |
| ---------- | ------------ | :------: | ----------- | ------------------- |
| disabled   | `boolean`    |          |             | Disables the input. |
| inputstate | `InputState` |          | `untouched` | State of the input. |
| name       | `string`     |          |             | Name of the input.  |
| required   | `boolean`    |          |             | Required attribute. |
| id         | `string`     |          |             | ID of the input.    |

## ValidationRule

A rule that can be used to validate an input. Either a `RegExp` or a `ValidationFunction`.

### Definition

```ts
type ValidationRule = ValidationFunction | RegExp
```

### ValidationFunction

A function that returns `InputState` based on input value and optionally state.

#### Definition

```ts
type ValidationFunction = (value: string, inputstate?: InputState) => InputState
```

See [Input Validation](/docs/components/input#validation) for more information.

## MoodValue

Numerical value from 1-5

### Definition

```ts
type MoodValue = 1 | 2 | 3 | 4 | 5
```
