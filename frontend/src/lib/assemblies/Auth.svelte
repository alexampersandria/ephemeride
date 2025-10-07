<script lang="ts">
import EmailInput from './EmailInput.svelte'
import PasswordInput from './PasswordInput.svelte'
import Input from '$lib/components/Input.svelte'
import Button from '$lib/components/Button.svelte'
import type { AuthModel } from '$lib/types/assemblies/auth'
import Message from '$lib/components/Message.svelte'
import { onMount } from 'svelte'
import { env } from '$env/dynamic/public'

let mode = $state<'login' | 'register'>('login')
let inviteRequired = $state(false)
let loading = $state(false)

let model: AuthModel = $state({
  name: {
    value: '',
    inputstate: 'untouched',
  },
  email: {
    value: '',
    inputstate: 'untouched',
  },
  password: {
    value: '',
    inputstate: 'untouched',
  },
  inviteCode: {
    value: '',
    inputstate: 'untouched',
  },
})

const isValid = $derived.by(() => {
  return (
    (mode === 'login' || model.name.inputstate === 'touched') &&
    model.email.inputstate === 'touched' &&
    model.password.inputstate === 'touched' &&
    (mode === 'login' || model.inviteCode.inputstate === 'touched')
  )
})

let disabled = $state(false)

$effect(() => {
  // React to model changes
  if (model && isValid) {
    disabled = false
  }
})

const switchMode = () => {
  mode = mode === 'login' ? 'register' : 'login'
}

const checkValidity = () => {
  if (!isValid) {
    disabled = true

    if (mode === 'register' && model.name.inputstate === 'untouched') {
      model.name.inputstate = 'invalid'
    }
    if (model.email.inputstate === 'untouched') {
      model.email.inputstate = 'invalid'
    }
    if (model.password.inputstate === 'untouched') {
      model.password.inputstate = 'invalid'
    }
    if (mode === 'register' && model.inviteCode.inputstate === 'untouched') {
      model.inviteCode.inputstate = 'invalid'
    }

    return
  }
}

const submit = async () => {
  checkValidity()
  if (disabled || loading) return

  loading = true

  if (mode === 'login') {
    // #TODO: Call login API
    console.log('Logging in with', model)
  } else {
    // #TODO: Call register API
    console.log('Registering with', model)
  }

  // fake loading time
  setTimeout(() => {
    loading = false
  }, 1000)
}

onMount(async () => {
  await fetch(env.PUBLIC_VITE_API_URL + '/v1/auth/config')
    .then(res => res.json())
    .then(data => {
      inviteRequired = data.invite_required
    })
    .catch(err => {
      console.error('Failed to fetch auth config:', err)
    })
})
</script>

<div class="auth-assembly">
  <div class="title">
    {#if mode === 'login'}
      Login
    {:else}
      Register
    {/if}
  </div>
  {#if mode === 'register'}
    <div class="form-field">
      <Input
        bind:value={model.name.value}
        bind:inputstate={model.name.inputstate}
        required
        placeholder="Name" />
    </div>
  {/if}
  <EmailInput
    bind:value={model.email.value}
    bind:inputstate={model.email.inputstate} />
  <PasswordInput
    bind:value={model.password.value}
    bind:inputstate={model.password.inputstate} />
  {#if mode === 'register' && inviteRequired}
    <div class="form-field">
      <Input
        bind:value={model.inviteCode.value}
        bind:inputstate={model.inviteCode.inputstate}
        required
        placeholder="Invite Code" />
    </div>
  {/if}

  <Button fullwidth type="primary" {disabled} {loading} onclick={submit}>
    {#if mode === 'login'}
      Login
    {:else}
      Register
    {/if}
  </Button>

  {#if disabled}
    <Message size="small" type="error">
      Please fill in all required fields.
    </Message>
  {/if}

  <p class="muted">
    {#if mode === 'login'}
      Don't have an account?
      <a href="#" onclick={switchMode}>Register</a>
    {:else}
      Already have an account?
      <a href="#" onclick={switchMode}>Login</a>
    {/if}
  </p>
</div>

<style lang="scss">
.auth-assembly {
  display: flex;
  flex-direction: column;
  gap: var(--form-gap);
  width: 100%;
  max-width: 24rem;
  margin: 0 auto;
  justify-content: center;
  align-items: center;

  .title {
    font-size: var(--font-size-xl);
    font-weight: 700;
  }
}
</style>
