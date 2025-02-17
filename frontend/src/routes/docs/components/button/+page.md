<script lang="ts">
import Button from '$lib/components/Button.svelte'
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import { Download } from "lucide-svelte"

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

HTML `<button>` element.

## Usage

Buttons are used to trigger actions or events.

### On Click


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

### Icon

An icon can be added to the button using lucide icons, see [Icons](/docs/design/icons) for more information.

<DocsExample>
  <Button>
    Download
    <Download />
  </Button>
</DocsExample>

```svelte
<script>
import { Download } from "lucide-svelte"
</script>

<Button>
  Download
  <Download />
</Button>
```

### Types
  
By default buttons are type `secondary`. `primary` buttons should be used for high priority actions or calls to action.

<DocsExample>
  <Button type='primary'>Primary</Button>
  <Button type='secondary'>Secondary</Button>
  <Button type='ghost'>Ghost</Button>
  <Button type='destructive'>Destructive</Button>
</DocsExample>

```svelte
<Button type="primary">Primary</Button>
<Button type="secondary">Secondary</Button>
<Button type="ghost">Ghost</Button>
<Button type='destructive'>Destructive</Button>
```

### Disabled

Disabling buttons should generally be avoided. All disabled buttons are styled the same regardless of type and color.

<DocsExample>
  <Button disabled type='primary'>Primary</Button>
  <Button disabled type='secondary'>Secondary</Button>
  <Button disabled type='ghost'>Ghost</Button>
  <Button disabled type='destructive'>Destructive</Button>
</DocsExample>

```svelte
<Button disabled type="primary">Primary</Button>
<Button disabled type="secondary">Secondary</Button>
<Button disabled type="ghost">Ghost</Button>
<Button disabled type='destructive'>Destructive</Button>
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
  <Button type='primary' onclick={incrementSlowCount} loading={slowCountLoading}>Click me</Button>
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
  type="primary"
  onclick={incrementSlowCount}
  loading={slowCountLoading}>
    Click me
</Button>
<p>Slow count: <code>{slowCount}</code></p>
```

## Types

### Props

| Name     | Type         | Required | Default       | Description                                      |
| -------- | ------------ | :------: | ------------- | ------------------------------------------------ |
| type     | `ButtonType` |          | `'secondary'` | Button type.                                     |
| disabled | `boolean`    |          | `false`       | Disables the button.                             |
| loading  | `boolean`    |          | `false`       | Shows a loading spinner and disables the button. |
| onclick  | `() => void` |          |               | Click event handler.                             |

## References

- [ButtonType](/docs/types/input#buttontype)
- [Icons](/docs/design/icons)
