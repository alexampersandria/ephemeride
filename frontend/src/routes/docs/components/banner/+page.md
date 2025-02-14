<script lang="ts">
import Banner from '$lib/components/Banner.svelte'
import Button from '$lib/components/Button.svelte'
import DocsExample from '$lib/components/utils/DocsExample.svelte'
</script>

# Banner

## Usage

### Default

<DocsExample>
  <Banner>
    Your account has been created. View your profile <a href="#profile">here</a>.
  </Banner>
</DocsExample>

```svelte
<Banner>
  Your account has been created. View your profile <a href="#profile">here</a>.
</Banner>
```

### Message & Actions

<DocsExample>
  <Banner>
    {#snippet message()}
      Your account has been created. View your profile <a href="#profile">here</a>.
    {/snippet}
    {#snippet actions()}
      <Button>Dismiss</Button>
      <Button>View Profile</Button>
    {/snippet}
  </Banner>
</DocsExample>

```svelte
<Banner>
  {#snippet message()}
    Your account has been created. View your profile <a href="#profile">here</a>.
  {/snippet}
  {#snippet actions()}
    <Button>Dismiss</Button>
    <Button>View Profile</Button>
  {/snippet}
</Banner>
```

### Variants

<DocsExample>
  <Banner variant="error">
    Email address is already in use. Try <a href="#forgot-password">forgot password</a>.
  </Banner>

  <Banner variant="warning">
    This action cannot be undone. Are you sure you want to proceed?
  </Banner>

  <Banner variant="success">
    Successfully updated your profile.
  </Banner>

  <Banner variant="info">
    Your account has been created. View your profile <a href="#profile">here</a>.
  </Banner>
</DocsExample>

```svelte
<Banner variant="error">
  Email address is already in use. Try <a href="#forgot-password">forgot password</a>.
</Banner>

<Banner variant="warning">
  This action cannot be undone. Are you sure you want to proceed?
</Banner>

<Banner variant="success">
  Successfully updated your profile.
</Banner>

<Banner variant="info">
  Your account has been created. View your profile <a href="#profile">here</a>.
</Banner>
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

- [NotificationState](/docs/types/notification#notificationstate)
