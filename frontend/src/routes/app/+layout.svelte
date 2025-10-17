<script lang="ts">
import { goto } from '$app/navigation'
import Button from '$lib/components/Button.svelte'
import Modal from '$lib/components/Modal.svelte'
import ThemeToggle from '$lib/components/ThemeToggle.svelte'
import { useUiStore } from '$lib/store/uiStore.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { currentDate, fullDate } from '$lib/utils/log'
import {
  CalendarDays,
  ChartColumnIncreasing,
  Github,
  LogOut,
  Menu,
  Pencil,
  User,
} from 'lucide-svelte'
import { onMount } from 'svelte'

let { children } = $props()

let userStore = useUserStore()
let uiStore = useUiStore()

const title = 'Ephemeride'

onMount(() => {
  if (!userStore.sessionId) {
    goto('/')
  }
})

let isDragging = $state(false)
let userDetailsModal = $state(false)

const startDrag = () => {
  isDragging = true

  const onMouseMove = (event: MouseEvent) => {
    uiStore.leftMenuWidth = Math.max(event.clientX, 128)
  }

  const onMouseUp = () => {
    isDragging = false
    window.removeEventListener('mousemove', onMouseMove)
    window.removeEventListener('mouseup', onMouseUp)
  }

  window.addEventListener('mousemove', onMouseMove)
  window.addEventListener('mouseup', onMouseUp)
}
</script>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<div
  class="app"
  class:left-menu-open={uiStore.leftMenuOpen}
  class:left-menu-dragging={isDragging}
  style="--app-left-menu-width: {uiStore.leftMenuWidth}px">
  <div class="top-bar">
    <div class="left">
      <Button
        type="ghost"
        onclick={() => (uiStore.leftMenuOpen = !uiStore.leftMenuOpen)}
        aria-label="Toggle Menu">
        <Menu />
      </Button>
    </div>

    <div class="right">
      {#if userStore.userDetails}
        <Button onclick={() => (userDetailsModal = true)}>
          <User />
          {userStore.userDetails.name}
        </Button>

        <Modal bind:open={userDetailsModal}>
          <div class="user-details">
            <div class="title">User Details</div>
            <div class="name">
              Display Name: {userStore.userDetails.name}
            </div>
            <div class="email">
              Email: {userStore.userDetails.email}
            </div>
            <div class="member-since">
              Member since: {fullDate(userStore.userDetails.created_at)}
            </div>

            <div class="logout">
              <Button
                onclick={() => {
                  userStore.logOut()
                }}
                fullwidth>
                Log out
              </Button>
            </div>
          </div>
        </Modal>
      {/if}

      <ThemeToggle />
    </div>
  </div>

  <div class="left-menu">
    <div class="items">
      <div class="actions">
        <Button type="ghost" href="/app/entry/{currentDate()}" left>
          <Pencil />
          <div class="ellipsis">Today</div>
        </Button>

        <Button type="ghost" href="/app/calendar" left>
          <CalendarDays />
          <div class="ellipsis">Calendar</div>
        </Button>

        <Button type="ghost" href="/app/stats" left>
          <ChartColumnIncreasing />
          <div class="ellipsis">Stats</div>
        </Button>
      </div>

      <div class="footer">
        <Button
          type="ghost"
          href="https://github.com/alexampersandria/ephemeride"
          target="_blank"
          left>
          <Github />
          <div class="ellipsis">GitHub</div>
        </Button>

        <Button type="ghost" onclick={() => userStore.logOut()} left>
          <LogOut />
          <div class="ellipsis">Log out</div>
        </Button>
      </div>
    </div>

    <div class="drag-area">
      <button onmousedown={startDrag} aria-label="Drag to resize left menu"
      ></button>
    </div>
  </div>

  <div class="content">
    {@render children()}
  </div>
</div>

<style lang="scss">
.app {
  --min-left-menu-width: 3.75rem;
  height: 100dvh;
  overflow: hidden;
  display: grid;
  grid-template-areas:
    'top-bar top-bar'
    'left-menu content';

  grid-template-rows: var(--app-top-bar-height) calc(
      100dvh - var(--app-top-bar-height)
    );
  grid-template-columns: var(--min-left-menu-width) 1fr;

  transition:
    grid-template-columns 0.15s ease-out,
    background-color 0.1s ease-out,
    border-color 0.1s ease-out;

  ::-webkit-scrollbar {
    background-color: transparent;
    width: 0.5rem;
    height: 0.5rem;
  }

  ::-webkit-scrollbar-track {
    background-color: var(--background-primary);
    border-radius: 9999px;
  }

  ::-webkit-scrollbar-thumb {
    background-color: var(--background-accent);
    border-radius: 9999px;
  }

  ::-webkit-scrollbar-corner {
    background-color: transparent;
  }

  &.left-menu-open {
    grid-template-columns:
      min(calc(var(--app-left-menu-width) - var(--padding-s)), 50vw)
      1fr;
  }

  @keyframes fadeOutEllipsis {
    0% {
      opacity: 1;
      display: block;
    }
    99% {
      opacity: 0;
      display: block;
    }
    100% {
      opacity: 0;
      display: none;
    }
  }

  @keyframes fadeInEllipsis {
    0% {
      opacity: 0;
      display: none;
    }
    1% {
      opacity: 0;
      display: block;
    }
    100% {
      opacity: 1;
      display: block;
    }
  }

  .items {
    .ellipsis {
      animation: fadeInEllipsis 0.1s ease-out;
      animation-fill-mode: forwards;
    }
  }

  &:not(.left-menu-open) {
    .items {
      .ellipsis {
        animation: fadeOutEllipsis 0.1s ease-out;
        animation-fill-mode: forwards;
      }
    }

    .drag-area {
      display: none;
    }
  }

  &.left-menu-dragging {
    transition: none;
    cursor: ew-resize;
    user-select: none;
  }

  background-color: var(--background-secondary);

  .content,
  .left-menu,
  .top-bar {
    transition:
      background-color 0.1s ease-out,
      border-color 0.1s ease-out;
  }

  .content {
    grid-area: content;
    background-color: var(--background-primary);
    border-radius: var(--radius-m);
    border: var(--border-width) solid var(--border-color);
    margin-right: var(--padding-s);
    margin-bottom: var(--padding-s);
    margin-left: var(--padding-s);
    overflow: auto;
  }

  .top-bar {
    grid-area: top-bar;
    display: flex;
    justify-content: space-between;
    padding: 0 var(--padding-s);

    .left,
    .right {
      display: flex;
      align-items: center;
      gap: var(--padding-m);
    }
  }

  .left-menu {
    grid-area: left-menu;
    position: relative;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding-bottom: var(--padding-m);
    overflow-x: visible;

    .items {
      display: flex;
      flex-direction: column;
      gap: var(--padding-s);
      padding-left: var(--padding-s);
      overflow: auto;
      justify-content: space-between;
      height: 100%;

      .actions,
      .footer {
        display: flex;
        flex-direction: column;
        gap: var(--padding-s);
      }
    }

    .drag-area {
      --drag-area-width: calc(var(--padding-s) * 2);
      position: absolute;
      top: 0;
      right: calc(
        -1 * var(--padding-s) - var(--drag-area-width) / 2 - var(--border-width)
      );
      width: var(--drag-area-width);
      height: calc(100% - var(--padding-s));
      border-radius: 9999px;
      cursor: ew-resize;

      button {
        all: unset;
        display: block;
        width: 100%;
        height: 100%;
      }
    }
  }
}

.user-details {
  display: flex;
  flex-direction: column;
  gap: var(--padding-xs);

  .title {
    font-size: var(--font-size-l);
    font-weight: 600;
  }

  .logout {
    margin-top: var(--padding-m);
  }
}
</style>
