# Input Types

## InputState

The state of the input, default should usually be `untouched`, components should automatically set to `touched` when interacted with via `$bindable`.

### Defition

```ts
type InputState = 'untouched' | 'touched' | 'valid' | 'invalid'
```
