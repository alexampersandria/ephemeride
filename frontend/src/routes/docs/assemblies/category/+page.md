<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Category from '$lib/assemblies/Category.svelte'

let tags = $state([
  {
    id: '1',
    name: 'Work',
    color: 'base'
  },
  {
    id: '2',
    name: 'Movie',
    color: 'pink'
  },
  {
    id: '3',
    name: 'Exercise',
    color: 'yellow'
  },
  {
    id: '4',
    name: 'Read',
    color: 'green'
  },
  {
    id: '5',
    name: 'Shopping',
    color: 'blue'
  },
  {
    id: '6',
    name: 'Gaming',
    color: 'red'
  },
])
</script>

# Category

still very much a WIP :)

## Usage

<DocsExample>
  <Category name="Activities" />
</DocsExample>

```svelte
<Category name="Activities" />
```

### Tags + Select

<DocsExample>
  <Category name="Activities" mode="select" tags={tags} />
</DocsExample>

```svelte
<script>
let tags = $state([
  {
    id: '1',
    name: 'Work',
    color: 'base',
  },
  …
  {
    id: '6',
    name: 'Gaming',
    color: 'red',
  },
])
</script>

<Category name="Activities" mode="select" {tags} />
```
