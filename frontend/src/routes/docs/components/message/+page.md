<script>
import Message from '$lib/components/Message.svelte';
import DocsExample from '$lib/components/utils/DocsExample.svelte';
</script>

# Message

A message is a simple component that displays a message to the user. Messages can be used to inform users of errors, warnings, successes, or general information.

## Usage

The message component should be used to convey information to the user. The message can be any content, including text, links, or lists, but should generally be concise and to the point and preferably be a single line.

### Default

By default, the message will be of type `info`.

<DocsExample>
  <Message>
    Heads up! This is a message.
  </Message>
</DocsExample>

```svelte
<Message>
  Heads up! This is a message.
</Message>
```

### Types

The message type determines the icon used and the color of the icon displayed to the left of the message.

#### All Types

<DocsExample>
  <Message type="error">
    An error occurred. Please try again.
  </Message>
</DocsExample>
<DocsExample>
  <Message type="warning">
    Heads up! This is a warning.
  </Message>
</DocsExample>
<DocsExample>
  <Message type="info">
    Heads up! This is an info message.
  </Message>
</DocsExample>
<DocsExample>
  <Message type="success">
    Heads up! This is a success message.
  </Message>
</DocsExample>

```svelte
<Message type="error">
  An error occurred. Please try again.
</Message>
<Message type="warning">
  Heads up! This is a warning.
</Message>
<Message type="info">
  Heads up! This is an info message.
</Message>
<Message type="success">
  Heads up! This is a success message.
</Message>
```

## Types

### Props

| Name | Type          | Required | Default | Description                                                         |
| ---- | ------------- | -------- | ------- | ------------------------------------------------------------------- |
| type | `MessageType` |          | `info`  | The type of alert. Either `error`, `warning`, `success`, or `info`. |

### Snippets

- `children()`: The message to display.

## References

- [MessageType](/docs/types/message#messagetype)
