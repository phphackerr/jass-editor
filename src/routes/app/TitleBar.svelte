<script>
  //@ts-nocheck
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import WindowMenu from "./WindowMenu.svelte";
</script>

<div data-tauri-drag-region class="titlebar">
  <div class="logo">
    <img src="" class="icon" alt="icon" />
    <WindowMenu />
  </div>
  <div class="text">
    <span data-tauri-drag-region class="span-text">Jass Editor</span>
  </div>
  <div class="buttons">
    <button
      class="titlebar-button"
      id="titlebar-minimize"
      onclick={getCurrentWindow().minimize}
      aria-label="Minimize"
    >
      <svg
        fill="#000000"
        width="24px"
        height="24px"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path d="M6 11H18V13H6V11Z" />
      </svg>
    </button>
    <button
      class="titlebar-button"
      id="titlebar-maximize"
      onclick={getCurrentWindow().toggleMaximize}
      aria-label="Maximize"
    >
      <svg
        fill="#000000"
        width="24px"
        height="24px"
        viewBox="-8.5 0 32 32"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M13.72 13.56c-0.48 0-0.84-0.36-0.84-0.84v-2.32h-2.32c-0.48 0-0.84-0.36-0.84-0.84s0.36-0.84 0.84-0.84v0h3.16c0.48 0 0.84 0.36 0.84 0.84v3.16c0 0.48-0.4 0.84-0.84 0.84zM13.72 23.28h-3.16c-0.48 0-0.84-0.36-0.84-0.84s0.36-0.84 0.84-0.84h2.32v-2.32c0-0.48 0.36-0.84 0.84-0.84v0c0.48 0 0.84 0.36 0.84 0.84v3.16c0 0.48-0.4 0.84-0.84 0.84zM4 23.28v0h-3.16c-0.48 0-0.84-0.36-0.84-0.84v-3.16c0-0.48 0.36-0.84 0.84-0.84s0.84 0.36 0.84 0.84v2.32h2.32c0.48 0 0.84 0.36 0.84 0.84s-0.4 0.84-0.84 0.84zM0.84 13.56v0c-0.48 0-0.84-0.36-0.84-0.84v-3.16c0-0.48 0.36-0.84 0.84-0.84h3.16c0.48 0 0.84 0.36 0.84 0.84s-0.36 0.84-0.84 0.84h-2.32v2.32c0 0.48-0.36 0.84-0.84 0.84z"
        ></path>
      </svg>
    </button>
    <button
      class="titlebar-button close"
      id="titlebar-close"
      onclick={getCurrentWindow().close}
      aria-label="Close"
    >
      <svg
        fill="#000000"
        width="24px"
        height="24px"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          clip-rule="evenodd"
          d="m7.53033 6.46967c-.29289-.29289-.76777-.29289-1.06066 0s-.29289.76777 0 1.06066l4.46963 4.46967-4.46963 4.4697c-.29289.2929-.29289.7677 0 1.0606s.76777.2929 1.06066 0l4.46967-4.4696 4.4697 4.4696c.2929.2929.7677.2929 1.0606 0s.2929-.7677 0-1.0606l-4.4696-4.4697 4.4696-4.46967c.2929-.29289.2929-.76777 0-1.06066s-.7677-.29289-1.0606 0l-4.4697 4.46963z"
          fill-rule="evenodd"
        />
      </svg>
    </button>
  </div>
</div>

<style>
  .titlebar {
    height: 30px;
    background: var(--titlebar-bg-color);
    user-select: none;
    display: flex;
    justify-content: space-between;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 100;
  }

  .logo {
    display: flex;
    align-items: center;
    padding-left: 4px;
  }

  .text {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .span-text {
    color: var(--titlebar-text);
    text-align: center;
    font-size: 14px;
    font-weight: 600 !important;
  }

  .icon {
    width: 26px;
    height: 26px;
  }

  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 45px;
    height: 30px;
    background: none;
    border: none;
  }

  .titlebar-button svg {
    fill: var(--titlebar-text);
  }

  .titlebar-button:hover {
    background: var(--titlebar-button-hover-color);
  }

  .titlebar-button:hover svg {
    fill: var(--titlebar-button-hover-color-svg);
  }

  .close:hover {
    background: var(--titlebar-button-close-hover);
  }

  .close:hover svg {
    fill: var(--titlebar-button-close-hover-svg);
  }
</style>
