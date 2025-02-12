<script lang="ts">
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Spinner from '$lib/components/Spinner.svelte'
</script>

## Spinner

Used to indicate that an action is being performed in the background.

---

### Default Size

<DocsExample>
  <Spinner />
</DocsExample>

```svelte
<Spinner />
```

---

### Custom Size

<DocsExample>
  <Spinner --spinner-size="1rem" />
</DocsExample>

```svelte
<Spinner --spinner-size="1rem" />
```

