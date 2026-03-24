/** @type {Record<string, string[]>} */
export const releaseNotes = {
  '0.4.5': [
    'Pull-to-refresh gesture on mobile',
    'Task tracking feature',
    'Fixed buttons being overlaid by Android system UI',
  ],
  '0.4.0': [
    'Stratz API added as an alternative to OpenDota',
    'Improved rate-limit handling and backoff logic',
    'Visual polish across the mobile layout',
    'Expanded Russian translations',
  ],
  '0.3.2': [
    'Full Russian language support across the app and landing page',
    'FAQ section added to the landing page',
    'Fixed duplicate refresh icons on Matches and Dashboard pages',
  ],
  '0.2.16': [
    'In-app feedback button — report bugs or request features without leaving the app',
    'Android build switched to AAB for smaller installs',
    'Beta build now installs as a separate app alongside production',
  ],
  '0.2.11': [
    'Match data now parses in the background — app stays responsive',
    'First-run onboarding flow for new users',
    'Factory reset option in Settings',
    'Fixed intermittent "database is locked" errors on startup',
  ],
  '0.2.7': [
    'Mind tab — track tilt and emotional state after games',
    'Steam Login via OpenID as an alternative login path',
    'Global toast notification system',
  ],
  '0.2.6': [
    'Custom title bar with drag-to-move and window controls',
    'Goal hero group filters — scope goals to Any Core, Any Carry, Any Support',
    'Denies and Partner Networth goal metrics',
  ],
  '0.2.4': [
    'Average performance line on goal distribution histogram',
    'Achievement rate card with colour-coded status (Too Easy → Critical)',
    'Warning banner when pass rate drops below 75% with one-click goal adjustment',
    'Last N games scatter chart with hover tooltips and click-to-match navigation',
  ],
  '0.1.10': [
    'Weekly Challenges — accept easy/medium/hard weekly goals with reroll support',
    'Challenge history page',
    'Edit goals inline from the goal details page',
    'Favourited heroes grouped at the top of hero dropdowns',
  ],
  '0.1.8': [
    'Match Details page — hero stats, KDA, GPM/XPM, damage, and last-hits chart',
    'Daily Challenges with streak counter and dashboard widget',
    'Item timing goal display with M:SS formatted times',
  ],
};
