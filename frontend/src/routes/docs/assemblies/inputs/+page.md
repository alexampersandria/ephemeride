<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import PasswordInput from '$lib/assemblies/PasswordInput.svelte'
import EmailInput from '$lib/assemblies/EmailInput.svelte'

let pinputstate = $state('untouched')
let pvalue = $state('')

let einputstate = $state('untouched')
let evalue = $state('')
</script>

# Inputs

Input assemblies provide an easy way to standardize and use complex input components. They include validation and error handling, making it easier to manage user input in forms.

## Password Input

A password input is an assembly that provides an Input component with password validation and error messages.

### Usage

<DocsExample>
  <PasswordInput bind:inputstate={pinputstate} bind:value={pvalue} />
</DocsExample>
<DocsExample column>
  <p style="margin: 0;">inputstate: <code>{pinputstate}</code></p>
  <p style="margin: 0;">value: <code>{pvalue}</code></p>
</DocsExample>

```svelte
<script>
let inputstate = $state('untouched')
let value = $state('')
</script>

<PasswordInput bind:inputstate bind:value />
```

## Email Input

An email input is an assembly that provides an Input component with email validation and error messages.

### Usage

<DocsExample>
  <EmailInput bind:inputstate={einputstate} bind:value={evalue} />
</DocsExample>
<DocsExample column>
  <p style="margin: 0;">inputstate: <code>{einputstate}</code></p>
  <p style="margin: 0;">value: <code>{evalue}</code></p>
</DocsExample>

```svelte
<script>
let inputstate = $state('untouched')
let value = $state('')
</script>

<EmailInput bind:inputstate bind:value />
```