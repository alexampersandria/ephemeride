<script lang="ts">
import Alert from '$lib/components/Alert.svelte'
import Input from '$lib/components/Input.svelte'
import type { PasswordInputProps } from '$lib/types/components/input'
import { validatePassword } from '$lib/utils/validationRules'

let {
  value = $bindable(),
  disabled,
  inputstate = $bindable('untouched'),
  fullwidth,
  live,
  name,
  id,
  required,
}: PasswordInputProps = $props()

let errors: string[] = $state([])
</script>

<div class="form-field">
  <Input
    type="password"
    validation={value => {
      return validatePassword(value, err => {
        errors = err
      })
    }}
    bind:value
    {disabled}
    bind:inputstate
    {name}
    {id}
    {required}
    {fullwidth}
    {live}
    placeholder="Password" />

  {#if errors.length > 0}
    <Alert size="small" type="error">
      <b>Password does not meet the following requirements:</b>
      <ul>
        {#each errors as error}
          <li>{error}</li>
        {/each}
      </ul>
    </Alert>
  {/if}
</div>
