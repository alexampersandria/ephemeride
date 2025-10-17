<script>
import Auth from '$lib/assemblies/Auth.svelte'
import Logo from '$lib/components/Logo.svelte'
</script>

<div class="login">
  <div class="logo">
    <a href="/"><Logo /></a>
  </div>

  <div class="auth-wrapper">
    <div class="container">
      <Auth />
    </div>
  </div>
</div>

<style lang="scss">
.login {
  height: 100vh;
  display: flex;
  flex-direction: column;

  .logo {
    padding: var(--padding-l) 0;
  }

  .auth-wrapper {
    flex: 1;
    display: flex;
    align-items: center;

    .container {
      width: 100%;
      margin-bottom: 8rem;
    }
  }
}
</style>
