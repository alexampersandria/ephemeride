<script lang="ts">
import Auth from '$lib/assemblies/Auth.svelte'
import Button from '$lib/components/Button.svelte'
import Logo from '$lib/components/Logo.svelte'
import Modal from '$lib/components/Modal.svelte'
import { useUiStore } from '$lib/store/uiStore.svelte'
import { Book, Moon, Sun, User, UserPlus } from 'lucide-svelte'

let authModal = $state(false)
let authMode = $state<'login' | 'register'>('register')
const openAuthModal = (mode: 'login' | 'register' = 'login') => {
  authModal = true
  authMode = mode
}

let uiStore = useUiStore()

const themeToggle = () => {
  if (uiStore.appliedTheme === 'dark') {
    uiStore.theme = 'light'
  } else {
    uiStore.theme = 'dark'
  }
}

const ThemeToggleIcon = $derived.by(() => {
  if (uiStore.appliedTheme === 'dark') {
    return Sun
  } else {
    return Moon
  }
})
</script>

<div class="landing-page">
  <Modal bind:open={authModal}>
    <Auth mode={authMode} />
  </Modal>

  <div class="navigation">
    <div class="container">
      <div class="left">
        <a class="fade-in fade-in-8" href="/"><Logo /></a>
      </div>
      <div class="right">
        <div class="fade-in fade-in-10">
          <Button type="ghost" href="/docs">
            <Book />
            Documentation
          </Button>
        </div>
        <div class="fade-in fade-in-12">
          <Button type="ghost" onclick={() => openAuthModal('login')}>
            <User />
            Log in
          </Button>
        </div>
        <div class="fade-in fade-in-14">
          <Button type="ghost" onclick={themeToggle} aria-label="Toggle Theme">
            <ThemeToggleIcon />
          </Button>
        </div>
      </div>
    </div>
  </div>

  <div class="container">
    <div class="header">
      <div class="text">
        <div class="fade-in fade-in-1 header-title instrument">
          Your life, documented
        </div>
        <div class="fade-in fade-in-2 muted small">
          Capture your life, day by day, and gain insights into how you live
        </div>
      </div>

      <div class="actions">
        <div class="fade-in fade-in-5">
          <Button type="primary" onclick={() => openAuthModal('register')}>
            <UserPlus />
            Sign up
          </Button>
        </div>

        <div class="fade-in fade-in-6">
          <Button type="ghost" href="/docs">
            <Book />
            Documentation
          </Button>
        </div>
      </div>
    </div>
  </div>
</div>

<style lang="scss">
.landing-page {
  background: linear-gradient(
    135deg,
    rgba(255, 182, 232, 0.05) 0%,
    rgba(173, 216, 230, 0.07) 25%,
    rgba(226, 236, 249, 0.05) 50%,
    rgba(240, 170, 245, 0.07) 75%,
    rgba(176, 184, 230, 0.05) 100%
  );
  min-height: 100vh;

  .navigation {
    position: fixed;
    width: 100%;

    .container {
      display: flex;
      justify-content: space-between;
      align-items: center;
      min-height: 4rem;
    }

    .left,
    .right {
      display: flex;
      align-items: center;
      gap: var(--padding-s);
    }
  }

  .header {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-height: var(--block-size-s);
    height: 95dvh;
    gap: var(--padding-l);

    .text {
      .header-title {
        font-size: var(--font-size-xl);
        font-weight: 600;
      }
    }

    .actions {
      display: flex;
      gap: var(--padding-s);
    }
  }

  .fade-in {
    animation: fade-in 0.66s ease-out;
    animation-fill-mode: backwards;

    @for $i from 1 through 99 {
      &.fade-in-#{$i} {
        animation-delay: calc($i * 0.1s + 0.25s);
      }
    }
  }

  @keyframes fade-in {
    0% {
      opacity: 0;
      transform: translateY(0.33rem);
      filter: blur(4px);
    }
    100% {
      opacity: 1;
      transform: translateY(0);
      filter: blur(0);
    }
  }
}
</style>
