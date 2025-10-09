<script>
import MoodInput from '$lib/components/MoodInput.svelte';
import DocsExample from '$lib/components/utils/DocsExample.svelte';

let value = $state()
</script>

# Mood Input

The mood input component is used on entry as a way for the user to rate their current mood on a scale of 1-5.

It features numbers 1-5 with an animated ruler that indicates the selected value and makes the component more engaging. The more enjoyable the core components feel to use, the more likely users are to engage with the app regularly, enabling better data analysis and results.

## Usage

The component features 5 values as buttons which change the bindable prop `value`.

<DocsExample column>
  <MoodInput bind:value={value} />
</DocsExample>

```svelte
<script>
let value = $state()
</script>

<MoodInput bind:value />
```

## Types

### Props

| Name  | Type        | Required | Default | Description    |
| ----- | ----------- | -------- | ------- | -------------- |
| value | `MoodValue` |          |         | Value from 1-5 |

## References

- [MoodValue](/docs/types/input#moodvalue)
