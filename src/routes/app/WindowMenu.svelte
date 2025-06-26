<script>
  //@ts-nocheck
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  let activeMenu = null;
  const win = getCurrentWindow();

  const menuItems = [
    {
      name: "Файл",
      submenu: [
        {
          text: "Открыть MPQ",
        },
        { isDivider: true },
        { text: "Выход", action: () => win.close() },
      ],
    },
    {
      name: "Правка",
      submenu: [{ text: "Отменить", action: () => console.log("Undo") }],
    },
  ];

  function toggleMenu(index) {
    activeMenu = activeMenu === index ? null : index;
  }

  function handleMouseEnter(index) {
    if (activeMenu !== null) {
      activeMenu = index;
    }
  }

  onMount(() => {
    const handleClickOutside = (event) => {
      const menuElement = document.querySelector(".custom-menu-bar");
      if (menuElement && !menuElement.contains(event.target)) {
        activeMenu = null;
      }
    };

    document.addEventListener("click", handleClickOutside);
    return () => document.removeEventListener("click", handleClickOutside);
  });
</script>

<div class="custom-menu-bar">
  {#each menuItems as item, index}
    <div
      class="menu-item {activeMenu === index ? 'active' : ''}"
      on:click={() => toggleMenu(index)}
      on:mouseenter={() => handleMouseEnter(index)}
      on:keydown={(e) => e.key === "Enter" && toggleMenu(index)}
      role="button"
      tabindex="0"
    >
      <span>{item.name}</span>
      {#if activeMenu === index}
        <div class="submenu">
          {#each item.submenu as subItem}
            {#if subItem.isDivider}
              <hr class="submenu-divider" />
            {:else}
              <button class="submenu-item" on:click={subItem.action}>
                {subItem.text}
              </button>
            {/if}
          {/each}
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  .custom-menu-bar {
    display: flex;
    height: 30px;
    align-items: center;
    z-index: 11;
  }

  .menu-item {
    padding: 5px 12px;
    cursor: default;
    position: relative;
    font-size: 13px;
    color: var(--titlebar-text);
    display: inline-flex;
    align-items: center;
  }

  .menu-item::before {
    content: "";
    position: absolute;
    top: 7px;
    left: 5px;
    right: 5px;
    bottom: 7px;
    border-radius: 6px;
    z-index: 1;
  }

  .menu-item:hover::before,
  .menu-item.active::before {
    background: var(--menu-item-hover-color);
  }

  .menu-item span {
    height: 100%;
    z-index: 2;
  }

  .menu-item:hover span,
  .menu-item.active span {
    color: var(--menu-item-hover-text-color);
  }

  .submenu {
    position: absolute;
    top: 100%;
    left: 0;
    background: var(--titlebar-bg-color);
    border: 1px solid var(--submenu-border-color);
    box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.1);
    z-index: 1000;
    min-width: 250px;
    display: flex;
    align-items: center;
    flex-direction: column;
    padding: 4px 0;
  }

  .submenu-item {
    padding: 6px 12px;
    width: 94%;
    border-radius: 5px;
    text-align: left;
    color: var(--text-color);
    background: none;
    border: none;
    cursor: pointer;
    color: var(--submenu-item-text-color);
  }

  .submenu-item:hover {
    color: var(--submenu-item-hover-text-color);
    background: var(--submenu-item-hover-color);
  }

  .submenu-divider {
    width: 98%;
    margin: 4px 0;
    border: none;
    border-top: 1px solid var(--submenu-divider-color);
  }
</style>
