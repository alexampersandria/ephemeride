<script>
import logo from '$lib/assets/img/Ephemeride_Logo.svg'
</script>

<div class="logo">
  <img src={logo} alt="Ephemeride Logo" />
</div>

<style lang="scss">
.logo {
  display: flex;
  justify-content: center;
  align-items: center;

  img {
    height: var(--logo-size, 1em);
  }
}
</style>
