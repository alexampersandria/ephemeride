<script lang="ts">
import Button from '$lib/components/Button.svelte'
import DocsExample from '$lib/components/utils/DocsExample.svelte'

let count = $state(0)

let slowCount = $state(0)
let slowCountLoading = $state(false)
let incrementSlowCount = () => {
  slowCountLoading = true
  setTimeout(() => {
    slowCount++
    slowCountLoading = false
  }, 1000)
}
</script>

# Button

## Usage

Buttons are used to trigger actions or events.

### Default

By default buttons are variant `secondary`. `primary` buttons should be used for high priority actions or calls to action.

<DocsExample>
  <Button onclick={() => {count++}}>Click me</Button>
</DocsExample>
<DocsExample>
  <p>Clicked <code>{count}</code> times</p>
</DocsExample>

```svelte
<script>
let count = $state(0)
</script>

<Button onclick={() => {count++}}>Click me</Button>
<p>Clicked <code>{count}</code> times</p>
```

### Variants

<DocsExample>
  <Button variant='primary'>Primary</Button>
  <Button variant='secondary'>Secondary</Button>
  <Button variant='ghost'>Ghost</Button>
  <Button variant='destructive'>Destructive</Button>
</DocsExample>

```svelte
<Button variant="primary">Primary</Button>
<Button variant="secondary">Secondary</Button>
<Button variant="ghost">Ghost</Button>
<Button variant='destructive'>Destructive</Button>
```

### Disabled

Disabling buttons should generally be avoided. All disabled buttons are styled the same regardless of variant and color.

<DocsExample>
  <Button disabled variant='primary'>Primary</Button>
  <Button disabled variant='secondary'>Secondary</Button>
  <Button disabled variant='ghost'>Ghost</Button>
  <Button disabled variant='destructive'>Destructive</Button>
</DocsExample>

```svelte
<Button disabled variant="primary">Primary</Button>
<Button disabled variant="secondary">Secondary</Button>
<Button disabled variant="ghost">Ghost</Button>
<Button disabled variant='destructive'>Destructive</Button>
```

### Loading

<DocsExample>
  <Button loading>Loading</Button>
</DocsExample>

```svelte
<Button loading>Loading</Button>
```

#### Loading interactivity

If a button is loading the onclick event will not be triggered.

<DocsExample>
  <Button variant='primary' onclick={incrementSlowCount} loading={slowCountLoading}>Click me</Button>
</DocsExample>
<DocsExample>
  <p>Slow count: <code>{slowCount}</code></p>
</DocsExample>

```svelte
<script>
let slowCount = $state(0)
let slowCountLoading = $state(false)
let incrementSlowCount = () => {
  slowCountLoading = true
  setTimeout(() => {
    slowCount++
    slowCountLoading = false
  }, 1000)
}
</script>

<Button
  variant="primary"
  onclick={incrementSlowCount}
  loading={slowCountLoading}>
    Click me
</Button>
<p>Slow count: <code>{slowCount}</code></p>
```

## Types

### Props

| Name     | Type            | Required | Default       | Description                                      |
| -------- | --------------- | :------: | ------------- | ------------------------------------------------ |
| variant  | `ButtonVariant` |          | `'secondary'` | Button variant.                                  |
| disabled | `boolean`       |          | `false`       | Disables the button.                             |
| loading  | `boolean`       |          | `false`       | Shows a loading spinner and disables the button. |
| onclick  | `() => void`    |          |               | Click event handler.                             |

## References

- [ButtonVariant](/docs/types/input#buttonvariant)
