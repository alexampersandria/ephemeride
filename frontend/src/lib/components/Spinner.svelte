<div class="spinner">
  <div class="spinner-container">
    {#each { length: 12 } as _, i}
      <div class="spinner-element"></div>
    {/each}
  </div>
</div>

<style lang="scss">
@use '../assets/scss/variables';

.spinner {
  height: var(--spinner-size);
  width: var(--spinner-size);

  .spinner-container {
    height: var(--spinner-size);
    width: var(--spinner-size);

    position: relative;
    top: 46%;
    left: 38%;

    .spinner-element {
      position: absolute;

      width: 24%;
      height: 8%;

      border-radius: var(--radius-s);

      background-color: var(--spinner-color);

      animation: spinner variables.$spinner-animation-duration infinite linear;

      @for $i from 1 through 12 {
        &:nth-child(#{$i}) {
          animation-delay: variables.$spinner-animation-duration * calc($i / 12) - 2;
          transform: rotate(#{$i * 30deg}) translate(150%);
        }
      }

      @keyframes spinner {
        0% {
          opacity: 1;
        }
        100% {
          opacity: 0.15;
        }
      }
    }
  }
}
</style>
