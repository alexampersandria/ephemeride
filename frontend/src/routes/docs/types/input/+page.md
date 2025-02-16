# Input Types

## ButtonVariant

The type or variant of the button. Default should always be `'secondary'`.

### Definition

```ts
type ButtonVariant = 'primary' | 'secondary' | 'ghost' | 'destructive'
```

## InputState

The state of the input, default should usually be `untouched`, components should automatically set to `touched` if `untouched` when interacted with via `$bindable`.

### Defition

```ts
type InputState = 'untouched' | 'touched' | 'valid' | 'invalid'
```

## FormElementProps

Shared props for form elements such as inputs, selects, and textareas.

| Name     | Type         | Required | Default     | Description         |
| -------- | ------------ | :------: | ----------- | ------------------- |
| disabled | `boolean`    |          |             | Disables the input. |
| state    | `InputState` |          | `untouched` | State of the input. |
| name     | `string`     |          |             | Name of the input.  |
| required | `boolean`    |          |             | Required attribute. |
| id       | `string`     |          |             | ID of the input.    |
