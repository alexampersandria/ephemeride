# Input Types

### ButtonVariant

The type or variant of the button. Default should always be `'secondary'`.

#### Definition

```ts
type ButtonVariant = 'primary' | 'secondary' | 'invisible'
```

## InputState

The state of the input, default should usually be `untouched`, components should automatically set to `touched` when interacted with via `$bindable`.

### Defition

```ts
type InputState = 'untouched' | 'touched' | 'valid' | 'invalid'
```
