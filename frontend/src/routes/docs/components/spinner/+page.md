<script lang="ts">
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Spinner from '$lib/components/Spinner.svelte'
</script>

# Spinner

Used to indicate that an action is being performed in the background.

## Usage

### Default

<DocsExample>
  <Spinner />
</DocsExample>

```svelte
<Spinner />
```

### Custom Size

Generally custom size should not be specified unless used in a specific context where the default size is not appropriate, such as in a button where the default size would be too big.

<DocsExample>
  <Spinner --spinner-size="1rem" />
</DocsExample>

```svelte
<Spinner --spinner-size="1rem" />
```
