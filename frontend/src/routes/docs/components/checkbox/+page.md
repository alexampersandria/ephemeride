<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Checkbox from '$lib/components/Checkbox.svelte'
import Label from '$lib/components/Label.svelte'
</script>

# Checkbox

## Usage

<DocsExample>
  <Checkbox name="terms" id="terms" />
  <Label for="terms" weight="normal">I have read and agree to the terms and conditions.</Label>
</DocsExample>

```svelte
<Checkbox name="terms" id="terms" />
<Label for="terms" weight="normal">I have read and agree to the terms and conditions.</Label>
```
