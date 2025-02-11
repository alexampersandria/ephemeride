<script lang="ts">
import type { Test } from '$lib/types/test'
import { uiState } from '$lib/store/ui.svelte'

import * as m from '$lib/paraglide/messages.js'

import { i18n } from '$lib/i18n'
import { page } from '$app/state'
import { goto } from '$app/navigation'
import type { AvailableLanguageTag } from '$lib/paraglide/runtime'

function switchToLanguage(newLanguage: AvailableLanguageTag) {
  const canonicalPath = i18n.route(page.url.pathname)
  const localisedPath = i18n.resolveRoute(canonicalPath, newLanguage)
  goto(localisedPath)
}

const test: Test = {
  title: 'Demo',
  paragraph: m.lorem(),
}
</script>

<div class="container">
  <h1>
    {m.hello({ variable: test.title })}
  </h1>
  <p>
    {test.paragraph}
  </p>

  <p>current theme: {uiState.theme}</p>

  <button onclick={() => switchToLanguage('en')}>English</button>

  <button
    onclick={() => {
      uiState.theme = 'DARK'
    }}
  >
    dark mode
  </button>

  <button
    onclick={() => {
      uiState.theme = 'LIGHT'
    }}
  >
    light mode
  </button>

  <button
    onclick={() => {
      uiState.theme = 'SYSTEM'
    }}
  >
    system default
  </button>

  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
  <p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem, officiis, ipsum omnis
    molestiae distinctio recusandae quam reiciendis vel culpa officia explicabo commodi dignissimos,
    rem excepturi debitis asperiores quis nobis provident!
  </p>
</div>
