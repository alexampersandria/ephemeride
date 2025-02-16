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
  <Alert type="success">
    {#snippet message()}
      Your account has been created. View your profile
    {/snippet}
    {#snippet actions()}
      <Button type="ghost">View Profile</Button>
      <Button>Dismiss</Button>
    {/snippet}
  </Alert>
</DocsExample>

```svelte
<Alert type="success">
  {#snippet message()}
    Your account has been created. View your profile
  {/snippet}
  {#snippet actions()}
    <Button type="ghost">View Profile</Button>
    <Button>Dismiss</Button>
  {/snippet}
</Alert>
```

### Types

The alerts type determines the icon used and the color of the icon displayed to the left of the message.

#### All Types

<DocsExample>
  <Alert type="error">
    An error occurred. Please try again.
  </Alert>
  <Alert type="warning">
    Email address is already in use. Try <a href="#forgot-password">forgot password</a>.
  </Alert>
  <Alert type="success">
    Successfully updated your profile.
  </Alert>
  <Alert type="info">
    You've got mail! Check your inbox.
  </Alert>
</DocsExample>

```svelte
<Alert type="error">
  An error occurred. Please try again.
</Alert>
<Alert type="warning">
  Email address is already in use. Try <a href="#forgot-password">forgot password</a>.
</Alert>
<Alert type="success">
  Successfully updated your profile.
</Alert>
<Alert type="info">
  You've got mail! Check your inbox.
</Alert>
```

#### Error with more info

An alert can render anything in its message content, including lists which can be used to provide more information.

<DocsExample>
  <Alert type="error">
    <b>Password does not meet requirements:</b>
    <ul class="text-muted">
      <li>Minimum 8 characters</li>
      <li>At least one uppercase letter</li>
    </ul>
  </Alert>
</DocsExample>

```svelte
<Alert type="error">
  <b>Password does not meet requirements:</b>
  <ul class="text-muted">
    <li>Minimum 8 characters</li>
    <li>At least one uppercase letter</li>
  </ul>
</Alert>
```

## Types

### Props

| Name | Type          | Required | Default | Description                                                         |
| ---- | ------------- | -------- | ------- | ------------------------------------------------------------------- |
| type | `MessageType` |          | `info`  | The type of alert. Either `error`, `warning`, `success`, or `info`. |

### Snippets

- `message()` / `children()`: The message content of the alert.
- `actions()`: The actions that can be performed.

## References

- [Icons](/docs/design/icons)
- [MessageType](/docs/types/message#messagetype)
