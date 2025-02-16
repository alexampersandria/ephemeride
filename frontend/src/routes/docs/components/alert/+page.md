<script lang="ts">
import Alert from '$lib/components/Alert.svelte'
import Button from '$lib/components/Button.svelte'
import DocsExample from '$lib/components/utils/DocsExample.svelte'
</script>

# Alert

## Usage

### Default

<DocsExample>
  <Alert>
    Heads up! This is an alert.
  </Alert>
</DocsExample>

```svelte
<Alert>
  Heads up! This is an alert.
</Alert>
```

### Message & Actions

<DocsExample>
  <Alert variant="success">
    {#snippet message()}
      Your account has been created. View your profile
    {/snippet}
    {#snippet actions()}
      <Button variant="ghost">View Profile</Button>
      <Button>Dismiss</Button>
    {/snippet}
  </Alert>
</DocsExample>

```svelte
<Alert variant="success">
  {#snippet message()}
    Your account has been created. View your profile
  {/snippet}
  {#snippet actions()}
    <Button variant="ghost">View Profile</Button>
    <Button>Dismiss</Button>
  {/snippet}
</Alert>
```

### Variant

The alerts variant determines the icon used and the color of the icon displayed to the left of the message.

#### All Variants

<DocsExample>
  <Alert variant="error">
    An error occurred. Please try again.
  </Alert>
  <Alert variant="warning">
    Email address is already in use. Try <a href="#forgot-password">forgot password</a>.
  </Alert>
  <Alert variant="success">
    Successfully updated your profile.
  </Alert>
  <Alert variant="info">
    You've got mail! Check your inbox.
  </Alert>
</DocsExample>

```svelte
<Alert variant="error">
  An error occurred. Please try again.
</Alert>
<Alert variant="warning">
  Email address is already in use. Try <a href="#forgot-password">forgot password</a>.
</Alert>
<Alert variant="success">
  Successfully updated your profile.
</Alert>
<Alert variant="info">
  You've got mail! Check your inbox.
</Alert>
```

#### Error with more info

An alert can render anything in its message content, including lists which can be used to provide more information.

<DocsExample>
  <Alert variant="error">
    <b>Password does not meet requirements:</b>
    <ul class="text-muted">
      <li>Minimum 8 characters</li>
      <li>At least one uppercase letter</li>
    </ul>
  </Alert>
</DocsExample>

```svelte
<Alert variant="error">
  <b>Password does not meet requirements:</b>
  <ul class="text-muted">
    <li>Minimum 8 characters</li>
    <li>At least one uppercase letter</li>
  </ul>
</Alert>
```

## Types

### Props

| Name    | Type                | Required | Default | Description                                                                     |
| ------- | ------------------- | -------- | ------- | ------------------------------------------------------------------------------- |
| variant | `NotificationState` |          | `info`  | The variant or type of banner. Either `error`, `warning`, `success`, or `info`. |

### Snippets

- `message()` / `children()`: The message content of the banner.
- `actions()`: The actions that can be performed.

## References

- [Icons](/docs/design/icons)
- [NotificationState](/docs/types/notification#notificationstate)
