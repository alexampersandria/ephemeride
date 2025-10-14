<script lang="ts">
import Alert from '$lib/components/Alert.svelte'
import Button from '$lib/components/Button.svelte'
import DocsExample from '$lib/components/utils/DocsExample.svelte'
</script>

# Alert

An alert is a message to the user that provides information or feedback. Alerts can be used to inform users of errors, warnings, successes, or general information.

Alerts use the [Message](/docs/components/message) component to display the message content, and can additionally include actions.

## Usage

The alert component is used to display a message to the user. The message can be any content, including text, links, or lists.

### Default

By default, the alert will be of type `info`.

<DocsExample>
  <Alert>
    Heads up! This is an alert.
  </Alert>
</DocsExample>

```svelte
<Alert>Heads up! This is an alert.</Alert>
```

### Message & Actions

Alerts can include actions in addition to the message using Snippets. Actions are aligned to the right of the alert.

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
<Alert type="error">An error occurred. Please try again.</Alert>
<Alert type="warning">
  Email address is already in use. Try <a href="#forgot-password">forgot password</a>.
</Alert>
<Alert type="success">Successfully updated your profile.</Alert>
<Alert type="info">You've got mail! Check your inbox.</Alert>
```

#### Error with more info

An alert can render anything in its message content, including lists which can be used to provide more information.

<DocsExample>
  <Alert type="error">
    <b>Password does not meet requirements:</b>
    <ul class="muted">
      <li>Minimum 8 characters</li>
      <li>At least one uppercase letter</li>
    </ul>
  </Alert>
</DocsExample>

```svelte
<Alert type="error">
  <b>Password does not meet requirements:</b>
  <ul class="muted">
    <li>Minimum 8 characters</li>
    <li>At least one uppercase letter</li>
  </ul>
</Alert>
```

### Size

The size of the alert can be adjusted using the `size` prop. The default size is `medium`. When used in combination with inputs or buttons to provide feedback, size `small` is should be used to match the size of the input or button.

<DocsExample>
  <Alert size="small">
    Heads up! This is a small alert.
  </Alert>
</DocsExample>

```svelte
<Alert size="small">Heads up! This is a small alert.</Alert>
```

## Types

### Props

| Name      | Type          | Required | Default  | Description                                                         |
| --------- | ------------- | -------- | -------- | ------------------------------------------------------------------- |
| type      | `MessageType` |          | `info`   | The type of alert. Either `error`, `warning`, `success`, or `info`. |
| colortext | `boolean`     |          | `false`  | Whether to color the text of the message.                           |
| size      | `MessageSize` |          | `medium` | The size of the alert. Either `small`, `medium`, or `large`.        |

### Snippets

- `message()` / `children()`: The message content of the alert.
- `actions()`: The actions that can be performed.

## References

- [Message](/docs/components/message)
- [Icons](/docs/design/icons)
- [MessageType](/docs/types/message#messagetype)
- [MessageSize](/docs/types/message#messagesize)
