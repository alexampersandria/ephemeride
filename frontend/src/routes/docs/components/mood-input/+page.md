<script>
import MoodInput from '$lib/components/MoodInput.svelte';
import DocsExample from '$lib/components/utils/DocsExample.svelte';

let value = $state()
</script>

# Mood Input

The mood input component is used on entry as a way for the user to rate their current mood on a scale of 1-5.

<DocsExample column>
  <MoodInput bind:value={value} />
  Value: { value }
</DocsExample>

```svelte
<MoodInput bind:value={value} />
Value: { value }
```

## Types

### Props

| Name  | Type        | Required | Default | Description    |
| ----- | ----------- | -------- | ------- | -------------- |
| value | `MoodValue` |          |         | Value from 1-5 |

## References

- [MoodValue](/docs/types/input#moodvalue)

