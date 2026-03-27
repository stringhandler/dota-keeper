# Login Screen Overflows on Mobile

## Priority
**HIGH** — First-run experience is broken on small screens

## Problem

The login box in [src/routes/+layout.svelte](src/routes/+layout.svelte) has a fixed width:

```css
.login-box {
  width: 420px;   /* ← overflows on screens < 420px */
  padding: 40px;
}
```

On a typical mobile screen (390px wide), the login box is wider than the viewport, causing horizontal overflow and a broken first impression.

Additionally:
- The Steam ID input has no `inputmode` attribute — mobile keyboards won't optimize for numeric entry
- The hint text (`11px`) is very small on mobile
- The 40px padding on all sides wastes vertical space on a small screen

## Fix

Add a mobile breakpoint to the login styles in [src/routes/+layout.svelte](src/routes/+layout.svelte):

```css
@media (max-width: 640px) {
  .login-box {
    width: 100%;
    max-width: 420px;
    padding: 28px 20px;
    border-radius: 0;  /* or keep but remove border if full-width */
    margin: 16px;
    width: calc(100% - 32px);
  }

  .login-brand {
    font-size: 22px;
  }
}
```

Also add `inputmode="numeric"` to the Steam ID input for numeric keyboards on mobile (Steam IDs are numeric):

```svelte
<input
  class="form-input"
  id="steam-id"
  type="text"
  inputmode="text"   <!-- keep text since profile URLs are also valid -->
  autocomplete="off"
  autocorrect="off"
  autocapitalize="none"
  placeholder="Steam ID or profile URL"
  bind:value={steamId}
/>
```

The `autocorrect="off"` and `autocapitalize="none"` attributes prevent iOS from mangling the numeric Steam ID.

## Acceptance Criteria

- [ ] Login screen displays correctly on a 390px wide screen (no horizontal scroll)
- [ ] Input field covers the full available width on mobile
- [ ] Keyboard on mobile doesn't auto-correct or capitalize the Steam ID input
- [ ] The "Save & Continue" button is easily tappable (already full-width, just verify it's ≥44px tall)
