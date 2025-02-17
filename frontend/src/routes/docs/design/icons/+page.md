<script>
import DocsExample from "$lib/components/utils/DocsExample.svelte"
import { Rocket } from "lucide-svelte"
</script>

# Icons

## Lucide Icons

This project uses [Lucide Icons](https://lucide.dev/), specifically the [lucide-svelte](https://lucide.dev/guide/packages/lucide-svelte) package to render SVG icons.

Icons can be imported and used as Svelte components, for example:

<DocsExample>
  <p>
    <Rocket /> Blast off!
  </p>
</DocsExample>

```svelte
<script>
import { Rocket } from "lucide-svelte"
</script>

<Rocket /> Blast off!
```

### Wrapper

In order for lucide icons to align with the text, a wrapper class `lucide-icon-wrapper` should be applied, this sets the height to zero to fix padding issues and lets the icon inherit the font size of the parent element, and sets it to `1.5em` which is lucide's default size. This class should be applied when the desired icon size is larger than the text.

Icons not inside a `lucide-icon-wrapper` will have a size of `1em` to align with text, this can be used in buttons or other components where the icon should be the same size as the text.

## References

- [Lucide Icons](https://lucide.dev/)
- [lucide-svelte](https://lucide.dev/guide/packages/lucide-svelte)
- [All Icons](https://lucide.dev/icons)
