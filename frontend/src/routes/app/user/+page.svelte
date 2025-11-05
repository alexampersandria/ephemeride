<script lang="ts">
import Spinner from '$lib/components/Spinner.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import type { Session } from '$lib/types/user'
import { getSessions } from '$lib/utils/api'
import { User } from 'lucide-svelte'
import { onMount } from 'svelte'

let userStore = useUserStore()

let sessions: Session[] | null = $state(null)

onMount(async () => {
  if (userStore.sessionId) {
    sessions = (await getSessions(userStore.sessionId)) || null
  }
})

let isActive = (session: Session): boolean => {
  return session.id === userStore.sessionId
}
</script>

<div class="app-page user-page">
  <div class="container">
    <div class="app-page-title">
      <User />
      Manage account (WIP)
    </div>

    <div class="section details">
      <div class="section-title">User Details</div>
      <table>
        <tbody>
          <tr>
            <th>Name</th>
            <td>{userStore.userDetails?.name}</td>
          </tr>
          <tr>
            <th>Email</th>
            <td>{userStore.userDetails?.email}</td>
          </tr>
          <tr>
            <th>Member Since</th>
            <td
              >{new Date(
                userStore.userDetails?.created_at || '',
              ).toLocaleString()}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="section sessions">
      <div class="section-title">Active Sessions</div>
      {#if sessions}
        <table>
          <thead>
            <tr>
              <th>Session ID</th>
              <th>Created At</th>
              <th>Accessed At</th>
              <th>IP Address</th>
              <th>User Agent</th>
            </tr>
          </thead>
          <tbody>
            {#each sessions as session}
              <tr class:active={isActive(session)}>
                <td>{session.id}</td>
                <td>{new Date(session.created_at).toLocaleString()}</td>
                <td>{new Date(session.accessed_at).toLocaleString()}</td>
                <td>{session.ip_address}</td>
                <td>{session.user_agent}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {:else}
        <div class="loading">
          <Spinner />
        </div>
      {/if}
    </div>
  </div>
</div>

<style lang="scss">
.user-page {
  .section {
    margin-bottom: var(--padding-l);

    .section-title {
      font-size: var(--font-size-m);
      font-weight: 600;
      margin-bottom: var(--padding-m);
    }

    table {
      width: 100%;
      border-collapse: collapse;

      th,
      td {
        text-align: left;
        padding: var(--padding-xs);
        border: 1px solid var(--border-color);
      }

      tr.active {
        background-color: var(--color-success-background);
      }
    }
  }
}
</style>
