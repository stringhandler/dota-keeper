<script>
  import { page } from "$app/stores";
  import { _ } from "svelte-i18n";

  const navItems = [
    {
      href: "/",
      labelKey: "home",
      exact: true,
      icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />`
    },
    {
      href: "/matches",
      labelKey: "matches",
      exact: false,
      icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />`
    },
    {
      href: "/analysis",
      labelKey: "analysis",
      exact: false,
      icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />`
    },
    {
      href: "/goals",
      labelKey: "goals",
      exact: false,
      icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />`
    },
    {
      href: "/mental-health",
      labelKey: "mind",
      exact: false,
      icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z" />`
    },
    {
      href: "/settings",
      labelKey: "settings",
      exact: false,
      icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />`
    }
  ];

  /** @param {{ href: string, exact: boolean }} item */
  function isActive(item) {
    if (item.exact) return $page.url.pathname === item.href;
    return $page.url.pathname.startsWith(item.href);
  }
</script>

<nav class="bottom-nav">
  {#each navItems as item}
    <a href={item.href} class="nav-tab" class:active={isActive(item)}>
      <svg class="tab-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
        {@html item.icon}
      </svg>
      <span class="tab-label">{$_('nav.' + item.labelKey)}</span>
    </a>
  {/each}
</nav>

<style>
  .bottom-nav {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    display: flex;
    background: var(--bg-card);
    border-top: 1px solid var(--border);
    padding-bottom: var(--sab);
    z-index: 100;
  }

  .nav-tab {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 10px 4px;
    gap: 4px;
    text-decoration: none;
    color: var(--text-secondary);
    transition: color 0.15s;
    min-height: 56px;
    -webkit-tap-highlight-color: transparent;
  }

  .nav-tab.active {
    color: var(--gold);
  }

  .tab-icon {
    width: 20px;
    height: 20px;
    opacity: 0.7;
    flex-shrink: 0;
  }

  .nav-tab.active .tab-icon {
    opacity: 1;
  }

  .tab-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.5px;
    text-transform: uppercase;
    text-align: center;
    white-space: normal;
    line-height: 1.1;
    word-break: break-word;
  }
</style>
