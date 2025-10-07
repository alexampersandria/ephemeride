<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Auth from '$lib/assemblies/Auth.svelte'
</script>

# Auth

Complete authentication interface with login and registration modes. Includes form validation and adapts to server configuration. Handles API calls and redirection internally.

## Usage

<DocsExample>
  <Auth />
</DocsExample>

```svelte
<Auth />
```

I mean... There's not a lot to it... 🤷‍♀️

## Types

These types are all internal, but can be useful for understanding how the assembly works.

### AuthModel

Authentication model containing all form fields.

| Name       | Type             | Description                     |
| ---------- | ---------------- | ------------------------------- |
| name       | `AuthModelField` | User's display name             |
| email      | `AuthModelField` | User's email address            |
| password   | `AuthModelField` | User's password                 |
| inviteCode | `AuthModelField` | Invite code (registration only) |

### AuthModelField

Individual form field with value and validation state.

| Name       | Type         | Description                   |
| ---------- | ------------ | ----------------------------- |
| value      | `string`     | Current field value           |
| inputstate | `InputState` | Validation state of the field |

## References

- [AuthModel](/docs/types/assemblies#authmodel)
- [EmailInput](/docs/assemblies/inputs#email-input)
- [PasswordInput](/docs/assemblies/inputs#password-input)
- [Input](/docs/components/input)
- [Button](/docs/components/button)
- [Message](/docs/components/message)
