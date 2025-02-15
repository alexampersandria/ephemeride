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
  <Button variant='invisible'>Invisible</Button>
</DocsExample>

```svelte
<Button variant="primary">Primary</Button>
<Button variant="secondary">Secondary</Button>
<Button variant="invisible">Invisible</Button>
```

### Disabled

Disabling buttons should generally be avoided. All disabled buttons are styled the same regardless of variant and color.

<DocsExample>
  <Button disabled color='base' variant='primary'>Primary</Button>
  <Button disabled color='red' variant='secondary'>Secondary</Button>
  <Button disabled color='pink' variant='invisible'>Invisible</Button>
</DocsExample>

```svelte
<Button disabled color="base" variant="primary">Primary</Button>
<Button disabled color="red" variant="secondary">Secondary</Button>
<Button disabled color="pink" variant="invisible">Invisible</Button>
```

### Color

Default color is `base`.

Colored buttons should be used sparingly for important actions, such as destructive actions or to indicate a confirmation at a glance.

#### Primary

<DocsExample>
  <Button variant='primary' color='base'>Base</Button>
  <Button variant='primary' color='blue'>Blue</Button>
  <Button variant='primary' color='green'>Green</Button>
  <Button variant='primary' color='red'>Red</Button>
  <Button variant='primary' color='yellow'>Yellow</Button>
  <Button variant='primary' color='purple'>Purple</Button>
  <Button variant='primary' color='pink'>Pink</Button>
</DocsExample>

```svelte
<Button variant="primary" color="base">Base</Button>
<Button variant="primary" color="blue">Blue</Button>
<Button variant="primary" color="green">Green</Button>
<Button variant="primary" color="red">Red</Button>
<Button variant="primary" color="yellow">Yellow</Button>
<Button variant="primary" color="purple">Purple</Button>
<Button variant="primary" color="pink">Pink</Button>
```

#### Secondary

<DocsExample>
  <Button variant='secondary' color='base'>Base</Button>
  <Button variant='secondary' color='blue'>Blue</Button>
  <Button variant='secondary' color='green'>Green</Button>
  <Button variant='secondary' color='red'>Red</Button>
  <Button variant='secondary' color='yellow'>Yellow</Button>
  <Button variant='secondary' color='purple'>Purple</Button>
  <Button variant='secondary' color='pink'>Pink</Button>
</DocsExample>

```svelte
<Button variant="secondary" color="base">Base</Button>
<Button variant="secondary" color="blue">Blue</Button>
<Button variant="secondary" color="green">Green</Button>
<Button variant="secondary" color="red">Red</Button>
<Button variant="secondary" color="yellow">Yellow</Button>
<Button variant="secondary" color="purple">Purple</Button>
<Button variant="secondary" color="pink">Pink</Button>
```

#### Invisible

<DocsExample>
  <Button variant='invisible' color='base'>Base</Button>
  <Button variant='invisible' color='blue'>Blue</Button>
  <Button variant='invisible' color='green'>Green</Button>
  <Button variant='invisible' color='red'>Red</Button>
  <Button variant='invisible' color='yellow'>Yellow</Button>
  <Button variant='invisible' color='purple'>Purple</Button>
  <Button variant='invisible' color='pink'>Pink</Button>
</DocsExample>

```svelte
<Button variant="invisible" color="base">Base</Button>
<Button variant="invisible" color="blue">Blue</Button>
<Button variant="invisible" color="green">Green</Button>
<Button variant="invisible" color="red">Red</Button>
<Button variant="invisible" color="yellow">Yellow</Button>
<Button variant="invisible" color="purple">Purple</Button>
<Button variant="invisible" color="pink">Pink</Button>
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
  <Button color='green' variant='primary' onclick={incrementSlowCount} loading={slowCountLoading}>Click me</Button>
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
  color="green"
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
| color    | `Color`         |          | `'base'`      | Button color.                                    |
| disabled | `boolean`       |          | `false`       | Disables the button.                             |
| loading  | `boolean`       |          | `false`       | Shows a loading spinner and disables the button. |
| onclick  | `() => void`    |          |               | Click event handler.                             |

## References

- [Colors](/docs/design/color)
- [ButtonVariant](/docs/types/input#buttonvariant)
