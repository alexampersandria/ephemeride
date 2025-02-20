<script lang="ts">
import type { FormFieldProps } from '$lib/types/components/formField'

const formFieldId = `form-field-${crypto.randomUUID()}`

let { children }: FormFieldProps = $props()
</script>

<div class="form-field">
  {@render children(formFieldId)}
</div>

<style lang="scss">
.form-field {
  display: flex;
  flex-direction: column;

  :global(.label) {
    padding-bottom: var(--form-field-gap);
  }
}
</style>
