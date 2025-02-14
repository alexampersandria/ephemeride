<script lang="ts">
const emojis = ['😿', '🤖', '💔', '🙇‍♀️', '⚠️']
const randomEmoji = emojis[Math.floor(Math.random() * emojis.length)]
</script>

<div class="error">
  <div class="error-content">
    <div class="emoji">{randomEmoji}</div>
    <div class="message">
      Womp womp, something went wrong.<br />
      Please try again later or go <a href="/">home</a>
    </div>
  </div>
</div>

<style lang="scss">
.error {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  width: 100vw;

  background-color: var(--color-base-10);
  color: var(--color-base-60);
  font-size: 1.5rem;
  font-weight: 700;

  .emoji {
    text-align: center;
    font-size: 3rem;
    padding: var(--padding-m);
  }
}
</style>
