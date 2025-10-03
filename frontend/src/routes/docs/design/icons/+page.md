<script>
import DocsExample from "$lib/components/utils/DocsExample.svelte"
import { Transgender } from "lucide-svelte"
</script>

# Icons

## Lucide Icons

This project uses [Lucide Icons](https://lucide.dev/), specifically the [lucide-svelte](https://lucide.dev/guide/packages/lucide-svelte) package to render SVG icons.

Icons can be imported and used as Svelte components, for example:

<DocsExample>
  <p>
    <Transgender /> Trans Rights!
  </p>
</DocsExample>

```svelte
<script>
import { Transgender } from "lucide-svelte"
</script>

<Transgender /> Trans Rights!
```

### Wrapper

The `lucide-icon-wrapper` class should be applied when you want icons to be `1.5em` in size (lucide's default). This class sets the height to zero to fix padding issues and allows the icon to inherit the font size of the parent element.

Icons not inside a `lucide-icon-wrapper` will have a size of `1em` to align with text, this can be used in buttons or other components where the icon should be the same size as the text.

<DocsExample column gap="3rem">
  <div class="lucide-icon-wrapper">
    <Transgender />
    Icon with wrapper (1.5em)
  </div>
  <div>
    <Transgender />
    Icon without wrapper (1em)
  </div>
</DocsExample>

```svelte
<div class="lucide-icon-wrapper">
  <Transgender />
  Icon with wrapper (1.5em)
</div>
<div>
  <Transgender />
  Icon without wrapper (1em)
</div>
```

## References

- [Lucide Icons](https://lucide.dev/)
- [lucide-svelte](https://lucide.dev/guide/packages/lucide-svelte)
- [All Icons](https://lucide.dev/icons)
