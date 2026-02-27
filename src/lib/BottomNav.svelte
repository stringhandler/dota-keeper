<script>
  import { page } from "$app/stores";

  const navItems = [
    {
      href: "/",
      label: "Home",
      exact: true,
      icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />`
    },
    {
      href: "/matches",
      label: "Matches",
      exact: false,
      icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />`
    },
    {
      href: "/goals",
      label: "Goals",
      exact: false,
      icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />`
    },
    {
      href: "/challenges",
      label: "Challenges",
      exact: false,
      icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z" />`
    },
    {
      href: "/settings",
      label: "Settings",
      exact: false,
      icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />`
    }
  ];

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
      <span class="tab-label">{item.label}</span>
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
    padding-bottom: env(safe-area-inset-bottom, 0px);
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
    font-size: 9px;
    font-weight: 600;
    letter-spacing: 1px;
    text-transform: uppercase;
  }
</style>
